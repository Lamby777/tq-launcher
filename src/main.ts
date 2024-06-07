"use strict";
import { invoke } from "@tauri-apps/api/tauri";

const newinstNameE = document.querySelector("#newinst-name") as HTMLInputElement;
const newinstVerE = document.querySelector("#newinst-ver") as HTMLSelectElement;
const instListE = document.querySelector("#instances") as HTMLDivElement;
const editPanelE = document.querySelector("#cpanel") as HTMLDivElement;
const newsE = document.querySelector("#tq-news") as HTMLDivElement;

type Release = any;
let releases: Release[];
let currentlyEditing: string;
let showEdgeBuilds = false;

async function main() {
    releases = await invoke("fetch_releases");

    repopulateReleases(releases);
    await repopulateInstanceRow();
}

try {
    main();
} catch (e) {
    console.error(e);
}

editPanelE.querySelector("#btn-delete")!
    .addEventListener("click", onDeletePressed);

editPanelE.querySelector("#cpanel-x-btn")!
    .addEventListener("click", hideEditPanel);

document.getElementById("edge-filter-check")!.addEventListener("change", (e) => {
    showEdgeBuilds = (e.target as HTMLInputElement).checked;
    repopulateReleases(releases);
});

document.querySelector("#newinst-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    createInstance();
});

let focusDebounce = false;
window.addEventListener("focus", async () => {
    if (focusDebounce) return;

    // update the instance list when the window is focused
    // this is in case the user deletes an instance folder or something
    focusDebounce = true;
    setTimeout(() => { focusDebounce = false; }, 1000);

    await repopulateInstanceRow();
});

async function createInstance() {
    const name = newinstNameE?.value;
    if (!name) return;

    if (newinstVerE?.selectedIndex === -1) return;

    const version = releases[newinstVerE.selectedIndex];

    // show modal so the user knows it's working
    openModal("Creating Instance", "Please wait...", []);

    await invoke("create_instance", {
        name,
        version,
    });

    closeModal();

    await repopulateInstanceRow();
}

function isEdgeBuildName(name: string) {
    return name.includes("-");
}

function repopulateReleases(releases: Release[]) {
    newinstVerE.innerHTML = "";

    for (const version of releases) {
        const name = version.name ?? "Unnamed";

        // skip edge builds if the user has the checkbox off
        if (!showEdgeBuilds && isEdgeBuildName(name)) continue;

        const option = document.createElement("option");
        option.innerText = name;
        newinstVerE.appendChild(option);
    }
}

function releaseNameFromId(id: string) {
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
        verE.innerText = releaseNameFromId(info?.release_id);

        const playButton = box.querySelector(".btn-play") as HTMLButtonElement;
        playButton.addEventListener("click", () => {
            invoke("play_instance", { name });
        });

        const editButton = box.querySelector(".btn-edit") as HTMLButtonElement;
        editButton.addEventListener("click", () => {
            currentlyEditing = name;
            editInstance();
        });

        instListE.appendChild(box);
    }
}

function onDeletePressed() {
    openModal("Delete Instance", "Are you sure you want to delete this instance?", [
        { text: "Cancel", onclick: () => { }, classes: ["btn-cancel"] },
        { text: "Delete!", onclick: deleteCurrentInst, classes: ["btn-delconf"] },
    ]);
}

function hideEditPanel() {
    editPanelE.style.display = "none";
    newsE.style.display = "block";
}

async function deleteCurrentInst() {
    if (!currentlyEditing) return;

    openModal("Deleting Instance", "Please wait...", []);

    await invoke("delete_instance", { name: currentlyEditing });
    await repopulateInstanceRow();

    hideEditPanel();
    closeModal();
}

function showEditPanel() {
    editPanelE.style.display = "block";
    newsE.style.display = "none";
}

function editInstance() {
    showEditPanel();

    const nameE = editPanelE.querySelector("#editing-inst-name") as HTMLHeadingElement;
    nameE.innerText = currentlyEditing;
}


function noInstancesBox() {
    return cloneTemplate("#instance-tmp-none")!;
}

function newInstanceBox(name: string) {
    const cloned = cloneTemplate("#instance-tmp")!;

    const nameH2 = cloned.querySelector(".instance-name") as HTMLHeadingElement;
    nameH2.textContent = name;

    return cloned;
}

interface ModalButtons {
    text: string;
    classes: string[];
    onclick: () => void;
}

function closeModal() {
    const modal = document.querySelector("#modal-bg") as HTMLDivElement;
    modal.style.display = "none";
    modal.remove();
}

function openModal(title: string, message: string, buttons: ModalButtons[]) {
    const cloned = cloneTemplate("#modal-tmp");
    document.body.prepend(cloned);

    const modal = document.querySelector("#modal-bg") as HTMLDivElement;

    modal.querySelector("#modal-title")!.textContent = title;
    modal.querySelector("#modal-text")!.textContent = message;

    for (const { text, onclick, classes } of buttons) {
        const button = document.createElement("button");
        button.classList.add("fancy-bg", ...classes);

        button.textContent = text;
        button.addEventListener("click", () => {
            modal.remove();
            onclick();
        });

        modal.querySelector("#modal-buttons")!.appendChild(button);
    }

    modal.style.display = "block";
}

function cloneTemplate(selector: string) {
    const tmp = document.querySelector(selector) as HTMLTemplateElement;
    return tmp.content.cloneNode(true) as HTMLElement;
}

