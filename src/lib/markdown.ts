import showdown from "showdown";

let converter = new showdown.Converter();

interface RawMesssage {
    timestamp: Date;
    role: string;
    content: string;
}

interface Message {
    timestamp: Date;
    role: string;
    chunks: Chunk[];
}

type Chunk = string | [string, string];

function htmlize(messages: RawMesssage[]): Message[] {
    if (!messages) {
        return [];
    }
    return messages.map((message) => {
        return toHighlightedMessage(message);
    });
}

function toHtml(markdown: string): string {
    return converter.makeHtml(markdown);
}

function toHighlightedMessage(message: RawMesssage): Message {
    let html = converter.makeHtml(message.content);
    let parsed = new DOMParser().parseFromString(html, "text/html");
    let chunks = [];
    for (let node of parsed.body.childNodes) {
        // if the node is a code block, we need to extract the language and the code,
        // code blocks are a <code> nested in a <pre> the language is stored as the first class
        // if the node is a not a code block, we just need to extract the html
        if (
            node.nodeName === "PRE" &&
            node.firstChild &&
            node.firstChild.nodeName === "CODE"
        ) {
            let code = node.firstChild.textContent;
            let language = node.firstChild.classList[0];
            chunks.push([language, code]);
        } else {
            const html = node.outerHTML;
            if (html) chunks.push(html);
        }
    }
    return { role: message.role, chunks };
}

export {
    htmlize,
    toHtml,
    toHighlightedMessage,
    type RawMesssage,
    type Message,
    type Chunk,
};
