import { pipeline } from "node:stream/promises";
import FormData from "form-data";
import got from "got";
import { ClientOptions } from "../index.js";
import { BufferWriteStream } from "../streams/buffer-write-stream.js";
import { MergeDto } from "../types/manipulations.js";
import { HttpService } from "./http-service.js";

export class ManipulationsService extends HttpService {
    constructor(options: ClientOptions) {
        super(options.serviceUrl, "manipulation");
    }

    public async merge(dto: MergeDto): Promise<Buffer> {
        const bufferWriteStream = new BufferWriteStream();

        await pipeline(this.streamedMerge(dto), bufferWriteStream);

        return bufferWriteStream.buffer;
    }

    public streamedMerge(dto: MergeDto): NodeJS.ReadableStream {
        const formData = new FormData();

        for (const document of dto.documents) {
            formData.append("documents", document.buffer, {
                filename: document.filename,
                contentType: document.mediaType,
            });
        }

        return got.stream.post(this.url("merge"), {
            body: formData,
        });
    }
}
