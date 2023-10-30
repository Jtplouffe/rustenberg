import FormData from "form-data";

export function formDataFromObject<T extends Record<string, unknown>>(
    obj: T,
    options?: { ignoreFields?: (keyof T)[] },
): FormData {
    const formData = new FormData();

    for (const [key, value] of Object.entries(obj)) {
        if (
            typeof value === "function" ||
            value === null ||
            value === undefined ||
            options?.ignoreFields?.includes(key)
        ) {
            continue;
        }

        formData.append(key, value);
        formData.append;
    }

    return formData;
}
