import showdown from "showdown";

import { highlightHtml } from "./code";

let converter = new showdown.Converter();

function toHtml(markdown: string): string {
    converter.makeHtml(markdown)
}

function toHighlightedHtml(markdown: string): string {
    let html = converter.makeHtml(markdown);
    return highlightHtml(html);
}

export { toHtml, toHighlightedHtml };
