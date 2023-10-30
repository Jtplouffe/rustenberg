import { pipeline } from "node:stream/promises";
import got from "got";
import { ClientOptions, ConvertHtmlDto, ConvertUrlDto } from "../index.js";
import { BufferWriteStream } from "../streams/buffer-write-stream.js";
import { formDataFromObject } from "../utils/form-data-utils.js";
import { HttpService } from "./http-service.js";

export class ConversionsService extends HttpService {
    constructor(clientOptions: ClientOptions) {
        super(clientOptions.serviceUrl, "conversion");
    }

    public async convertUrl(dto: ConvertUrlDto): Promise<Buffer> {
        const bufferWriteStream = new BufferWriteStream();

        await pipeline(this.streamedConvertUrl(dto), bufferWriteStream);

        return bufferWriteStream.buffer;
    }

    public streamedConvertUrl(dto: ConvertUrlDto): NodeJS.ReadableStream {
        const formData = formDataFromObject(dto);

        return got.stream.post(this.url("url"), {
            body: formData,
        });
    }

    public async convertHtml(dto: ConvertHtmlDto): Promise<Buffer> {
        const bufferWriteStream = new BufferWriteStream();

        await pipeline(this.streamedConvertHtml(dto), bufferWriteStream);

        return bufferWriteStream.buffer;
    }

    public streamedConvertHtml(dto: ConvertHtmlDto): NodeJS.ReadableStream {
        const formData = formDataFromObject(dto, { ignoreFields: ["files"] });

        for (const file of dto.files) {
            formData.append("files", file.buffer, { filename: file.filename, contentType: file.mediaType });
        }

        return got.stream.post(this.url("html"), {
            body: formData,
        });
    }
}
