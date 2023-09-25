use std::collections::BTreeMap;

use anyhow::anyhow;
use lopdf::{Document, Object, ObjectId};

pub fn merge(raw_documents: &[Vec<u8>]) -> anyhow::Result<Vec<u8>> {
    if raw_documents.len() < 2 {
        return Err(anyhow!(
            "multiple documents must be provided in order to merge"
        ));
    }

    let documents = raw_documents
        .iter()
        .map(|raw_document| Document::load_mem(raw_document))
        .collect::<Result<Vec<_>, _>>()?;

    let mut current_max_object_id = 0;

    let mut documents_pages = BTreeMap::new();
    let mut documents_objects = BTreeMap::new();

    for mut document in documents {
        document.renumber_objects_with(current_max_object_id);
        current_max_object_id = document.max_id + 1;

        documents_pages.extend(document.get_pages().into_values().map(|object_id| {
            (
                object_id,
                document.get_object(object_id).unwrap().to_owned(),
            )
        }));
        documents_objects.extend(document.objects);
    }

    let mut merged_document = Document::with_version("1.5");

    let mut catalog_object: Option<(ObjectId, Object)> = None;
    let mut pages_object: Option<(ObjectId, Object)> = None;

    for (object_id, object) in documents_objects {
        match object.type_name().unwrap_or("") {
            "Catalog" => {
                catalog_object = Some((
                    if let Some((id, _)) = catalog_object {
                        id
                    } else {
                        object_id
                    },
                    object,
                ))
            }
            "Pages" => {
                if let Ok(object_dictionary) = object.as_dict() {
                    let mut object_dictionary = object_dictionary.clone();

                    if let Some((_, ref object)) = pages_object {
                        if let Ok(old_object_dictionary) = object.as_dict() {
                            object_dictionary.extend(old_object_dictionary);
                        }
                    }

                    pages_object = Some((
                        if let Some((id, _)) = pages_object {
                            id
                        } else {
                            object_id
                        },
                        Object::Dictionary(object_dictionary),
                    ))
                }
            }
            "Page" => {}
            _ => {
                merged_document.objects.insert(object_id, object);
            }
        }
    }

    let (pages_object_id, pages_object) = match pages_object {
        Some(pages_object) => pages_object,
        None => return Err(anyhow!("Pages object not found")),
    };

    let (catalog_object_id, catalog_object) = match catalog_object {
        Some(catalog_object) => catalog_object,
        None => return Err(anyhow!("Catalog root not found")),
    };

    for (object_id, object) in documents_pages.iter() {
        if let Ok(object_dictionary) = object.as_dict() {
            let mut object_dictionary = object_dictionary.clone();
            object_dictionary.set("Parent", pages_object_id);
            merged_document
                .objects
                .insert(*object_id, Object::Dictionary(object_dictionary));
        }
    }

    if let Ok(dictionary) = pages_object.as_dict() {
        let mut dictionary = dictionary.clone();
        dictionary.set("Count", documents_pages.len() as u32);
        dictionary.set(
            "Kids",
            documents_pages
                .into_keys()
                .map(Object::Reference)
                .collect::<Vec<_>>(),
        );
        merged_document
            .objects
            .insert(pages_object_id, Object::Dictionary(dictionary));
    }

    if let Ok(catalog_object_dictionary) = catalog_object.as_dict() {
        let mut catalog_object_dictionary = catalog_object_dictionary.clone();
        catalog_object_dictionary.set("Pages", pages_object_id);
        catalog_object_dictionary.remove(b"Outlines");
        merged_document.objects.insert(
            catalog_object_id,
            Object::Dictionary(catalog_object_dictionary),
        );
    }

    merged_document.trailer.set("Root", catalog_object_id);
    merged_document.max_id = merged_document.objects.len() as u32;
    merged_document.renumber_objects();
    merged_document.compress();

    let mut raw_merged_document = Vec::<u8>::new();
    merged_document.save_to(&mut raw_merged_document)?;

    Ok(raw_merged_document)
}
