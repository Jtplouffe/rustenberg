import { File } from "./file.js";

export interface MergeDto {
    documents: (File & { mediaType?: "application/pdf" })[];
}
