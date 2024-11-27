import { invoke } from "@tauri-apps/api/core";

function modelListToMapWithIndexId(
  models: { engine: string; name: string }[],
): { id: string; engine: string; name: string }[] {
  let map = [];
  for (let i = 0; i < models.length; i++) {
    map[i] = {
      id: i.toString(),
      ...models[i],
    };
  }
  return map;
}

async function getModels(): Promise<{ engine: string; name: string }[]> {
  const rawModels: { engine: string; name: string }[] =
    await invoke("list_models");
  // return modelListToMapWithIndexId(rawModels);
  return rawModels;
}

export default { getModels };
