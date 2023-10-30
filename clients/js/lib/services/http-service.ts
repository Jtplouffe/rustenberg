export class HttpService {
    private baseUrl: string;

    constructor(serviceUrl: string, path: string) {
        if (serviceUrl.endsWith("/")) {
            this.baseUrl = serviceUrl.substring(0, serviceUrl.length - 1);
        } else {
            this.baseUrl = serviceUrl;
        }

        if (path.startsWith("/")) {
            this.baseUrl += path;
        } else {
            this.baseUrl += `/${path}`;
        }

        if (this.baseUrl.endsWith("/")) {
            this.baseUrl = this.baseUrl.substring(0, this.baseUrl.length - 1);
        }
    }

    protected url(path?: string): string {
        if (!path) {
            return this.baseUrl;
        }

        let url = this.baseUrl;

        if (path.startsWith("/")) {
            url += path;
        } else {
            url += `/${path}`;
        }

        if (url.endsWith("/")) {
            url = url.substring(0, url.length - 1);
        }

        return url;
    }
}
