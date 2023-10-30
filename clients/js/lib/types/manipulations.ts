import { File } from "./file.js";

export type MergeDto = {
    documents: (File & { mediaType?: "application/pdf" })[];
};
