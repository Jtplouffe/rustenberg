import { File } from "./file.js";

type BaseConversionDto = {
    landscape?: boolean;
    displayHeaderFooter?: boolean;
    printBackground?: boolean;
    scale?: number;
    paperWidth?: number;
    paperHeight?: number;
    marginTop?: number;
    marginBottom?: number;
    marginLeft?: number;
    marginRight?: number;
    pageRange?: string;
    headerTemplate?: string;
    footerTemplate?: string;
    preferCssPageSize?: boolean;
    minPageLoadTimeMs?: number;
    maxPageLoadTimeMs?: number;
};

export type ConvertUrlDto = BaseConversionDto & {
    url: string;
};

export type ConvertHtmlDto = BaseConversionDto & {
    files: File[];
};
