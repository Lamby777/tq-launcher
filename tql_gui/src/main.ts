import { invoke } from "@tauri-apps/api/tauri";

const newinstNameE = document.querySelector("#newinst-name") as HTMLInputElement;
const newinstVerE = document.querySelector("#newinst-ver") as HTMLSelectElement;
const instListE = document.querySelector("#instances") as HTMLDivElement;

type Release = any;
const releases: Release[] = await invoke("fetch_releases");

function main() {
    populateReleases(releases);
    repopulateInstanceRow();
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

    await repopulateInstanceRow();
}

document.querySelector("#newinst-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    createInstance();
});

let focus_debounce = false;
// update the instance list when the window is focused
// this is in case the user deletes an instance folder or something
window.addEventListener("focus", async () => {
    if (focus_debounce) return;

    focus_debounce = true;
    setTimeout(() => { focus_debounce = false; }, 1000);

    console.log("Window focused");
    await repopulateInstanceRow();
});

function populateReleases(releases: Release[]) {
    for (const version of releases) {
        const name = version.name ?? "Unnamed";

        const option = document.createElement("option");
        option.innerText = name;
        newinstVerE.appendChild(option);
    }
}

async function repopulateInstanceRow() {
    instListE.innerHTML = "";

    const instances: any = await invoke("instance_names");

    if (instances.length === 0) {
        const box = noInstancesBox();
        instListE.appendChild(box);
        return;
    }

    for (const name of instances) {
        const box = newInstanceBox(name);

        const play_button = box.querySelector(".btn-play") as HTMLButtonElement;
        play_button.onclick = () => {
            invoke("play_instance", { name });
        };

        instListE.appendChild(box);
    }
}

function noInstancesBox() {
    const tmp = document.getElementById("instance-template-none") as HTMLTemplateElement;
    return tmp.content.cloneNode(true) as HTMLDivElement;
}

function newInstanceBox(name: string) {
    const tmp = document.getElementById("instance-template") as HTMLTemplateElement;
    const cloned = tmp.content.cloneNode(true) as HTMLDivElement;

    const name_h2 = cloned.querySelector(".instance-name") as HTMLHeadingElement;
    name_h2.textContent = name;

    return cloned;
}
