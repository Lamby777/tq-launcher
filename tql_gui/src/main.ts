import { invoke } from "@tauri-apps/api/tauri";

const newinstNameE = document.querySelector("#newinst-name") as HTMLInputElement;
const newinstVerE = document.querySelector("#newinst-ver") as HTMLSelectElement;

type Release = any;
const releases: Release[] = await invoke("fetch_releases");
populateVersions(releases);

async function createInstance() {
    const name = newinstNameE?.value;
    if (!name) return;

    if (newinstVerE?.selectedIndex === -1) return;

    const version = releases[newinstVerE.selectedIndex];

    await invoke("create_instance", {
        name,
        version,
    });
}

document.querySelector("#newinst-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    createInstance();
});

function populateVersions(releases: Release[]) {
    for (const version of releases) {
        const name = version.name ?? "Unnamed";

        const option = document.createElement("option");
        option.innerText = name;
        newinstVerE.appendChild(option);
    }
}
