"use strict";
import { invoke } from "@tauri-apps/api/tauri";

const newinstNameE = document.querySelector("#newinst-name") as HTMLInputElement;
const newinstVerE = document.querySelector("#newinst-ver") as HTMLSelectElement;
const instListE = document.querySelector("#instances") as HTMLDivElement;

type Release = any;
let releases: Release[];
let currentlyEditing: string;

async function main() {
    releases = await invoke("fetch_releases");

    populateReleases(releases);
    await repopulateInstanceRow();
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

function release_name_from_id(id: string) {
    return releases.find((r) => r.id === id)?.name;
}

async function repopulateInstanceRow() {
    instListE.innerHTML = "";

    const instances: any = await invoke("instance_map");

    if (Object.keys(instances).length === 0) {
        const box = noInstancesBox();
        instListE.appendChild(box);
        return;
    }

    for (const [name, info] of Object.entries(instances) as any) {
        const box = newInstanceBox(name);

        const verE = box.querySelector(".instance-ver") as HTMLHeadingElement;
        verE.innerText = release_name_from_id(info?.release_id);

        const play_button = box.querySelector(".btn-play") as HTMLButtonElement;
        play_button.addEventListener("click", () => {
            invoke("play_instance", { name });
        });

        const edit_button = box.querySelector(".btn-edit") as HTMLButtonElement;
        edit_button.addEventListener("click", () => {
            currentlyEditing = name;
            edit_instance();
        });

        instListE.appendChild(box);
    }
}

async function onDeletePressed() {
    if (!currentlyEditing) return;

    await invoke("delete_instance", { name: currentlyEditing });
    await repopulateInstanceRow();
}

function edit_instance() {
    const cpanel = get_or_make_cpanel();

    const nameE = cpanel.querySelector("#editing-inst-name") as HTMLHeadingElement;
    nameE.innerText = currentlyEditing;
}

function get_or_make_cpanel() {
    const existing = document.querySelector("#cpanel") as HTMLDivElement;
    if (existing) return existing;

    // if not, delete the news panel and make an edit one
    const news = document.querySelector("#tq-news") as HTMLDivElement;
    news.remove();

    const tmp = document.getElementById("cpanel-template") as HTMLTemplateElement;
    const cloned = tmp.content.cloneNode(true) as HTMLDivElement;
    instListE.parentNode!.appendChild(cloned);

    const parent = instListE.parentNode!;
    parent.appendChild(cloned);
    const panel = parent.querySelector("#cpanel")!;


    const delete_button = document.getElementById("btn-delete")!;
    delete_button.addEventListener("click", onDeletePressed);

    return panel;
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
