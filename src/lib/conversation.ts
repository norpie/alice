import { htmlize } from "$lib/markdown";
import { invoke } from "@tauri-apps/api/core";

interface RawConversation {
    id: string;
    name: string;
    start_time: Date;
    modified_time: Date;
    messages: RawMesssage[];
}

interface Conversation {
    id: string;
    name: string;
    start_time: Date;
    modified_time: Date;
    messages: Message[];
}

function convert(rawConversation: RawConversation): Conversation {
    return {
        id: rawConversation.id.id.String,
        name: rawConversation.name,
        start_time: Date.parse(rawConversation.start_time),
        modified_time: Date.parse(rawConversation.modified_time),
        messages: htmlize(rawConversation.messages),
    };
}

const groups = [
    { name: "Today", after: Date.now() - 86400000 },
    { name: "Yesterday", after: Date.now() - 172800000 },
    { name: "Older", after: Date.parse("1970-01-01") },
];

type GroupedConversations = {
    name: string;
    conversations: Conversation[];
}[];

function dategrouped(conversations: Conversation[]): GroupedConversations {
    const grouped = [];
    const used = [];
    for (let i = 0; i < groups.length; i++) {
        const found = [];
        for (let j = 0; j < conversations.length; j++) {
            if (
                conversations[j].modified_time > groups[i].after &&
                !used.includes(j)
            ) {
                found.push(conversations[j]);
                used.push(j);
            }
        }
        grouped[i] = {
            name: groups[i].name,
            conversations: found,
        };
    }
    return grouped;
}

async function find(id: string | null): Promise<Conversation | null> {
    if (!id) return null;
    return convert(await invoke("find_conversation", { id }));
}

async function getConversations(
    limit: number,
    offset: number,
): Promise<Conversation[]> {
    const rawConversations: RawConversation[] = await invoke(
        "conversations_date_sorted",
        { limit: limit, offset: offset },
    );
    let conversations = [];
    for (let i = 0; i < rawConversations.length; i++) {
        let converted = convert(rawConversations[i]);
        conversations[i] = converted;
    }
    return conversations;
}

export {
    convert,
    dategrouped,
    getConversations,
    find,
    type GroupedConversations,
    type Conversation,
    type RawConversation,
};
