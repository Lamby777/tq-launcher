import { invoke } from "@tauri-apps/api/tauri";

const newinstNameE = document.querySelector("#newinst-name") as HTMLInputElement;
const newinstVerE = document.querySelector("#newinst-ver") as HTMLSelectElement;
const instListE = document.querySelector("#instances") as HTMLDivElement;

type Release = any;
const releases: Release[] = await invoke("fetch_releases");

function main() {
    populateReleases(releases);

    newInstanceBox("Hello World");
    newInstanceBox("Hi World");
}

try {
    main();
} catch (e) {
    console.error(e);
}

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

function populateReleases(releases: Release[]) {
    for (const version of releases) {
        const name = version.name ?? "Unnamed";

        const option = document.createElement("option");
        option.innerText = name;
        newinstVerE.appendChild(option);
    }
}

function newInstanceBox(name: string) {
    const tmp = document.getElementById("instance-template") as HTMLTemplateElement;
    const cloned = tmp.content.cloneNode(true) as HTMLDivElement;

    const name_h2 = cloned.querySelector(".instance-name") as HTMLHeadingElement;
    name_h2.textContent = name;

    instListE.appendChild(cloned);
}
