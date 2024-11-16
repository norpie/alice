import { invoke } from "@tauri-apps/api/core";

import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ params }) => {
  let alive = await invoke("connection_status");
  return { alive, params };
};
