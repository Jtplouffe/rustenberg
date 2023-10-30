import { Writable } from "stream";

export class BufferWriteStream extends Writable {
    public buffer = Buffer.alloc(0);

    // biome-ignore lint/suspicious/noExplicitAny: inherited function typings
    public _write(chunk: any, _: BufferEncoding, callback: (error?: Error | null | undefined) => void): void {
        this.buffer = Buffer.concat([this.buffer, chunk]);
        callback();
    }
}
