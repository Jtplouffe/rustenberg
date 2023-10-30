import got from "got";
import { ClientOptions, ManipulationsService, ServiceInfo } from "./index.js";
import { ConversionsService } from "./services/conversions-service.js";
import { HttpService } from "./services/http-service.js";

export class Client extends HttpService {
    private _conversions?: ConversionsService;
    private _manipulations?: ManipulationsService;

    public get conversions(): ConversionsService {
        this._conversions ??= new ConversionsService(this.options);

        return this._conversions;
    }

    public get manipulations(): ManipulationsService {
        this._manipulations ??= new ManipulationsService(this.options);

        return this._manipulations;
    }

    constructor(private readonly options: ClientOptions) {
        super(options.serviceUrl, "");
    }

    public getServiceInfo(): Promise<ServiceInfo> {
        return got.get(this.url()).json();
    }
}
