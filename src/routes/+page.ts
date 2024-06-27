import { invoke } from "@tauri-apps/api/core";

import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ params }) => {
  let status = await invoke("check_connection");
  return { status, params };
};
