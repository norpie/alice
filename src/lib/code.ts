import hljs from "highlight.js";

function highlightHtml(html: string): string {
    let parser = new DOMParser();
    let doc = parser.parseFromString(html, "text/html");
    let inlines = doc.querySelectorAll("p > code");
    inlines.forEach((inline) => {
        hljs.highlightBlock(inline as HTMLElement);
    });
    let blocks = doc.querySelectorAll("pre code");
    blocks.forEach((block) => {
        hljs.highlightBlock(block as HTMLElement);
    });
    return doc.body.innerHTML;
}

export { highlightHtml };
