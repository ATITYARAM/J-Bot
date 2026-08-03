const container = document.getElementById("process-container");

let processes = [];

/* ==========================================================
   LOAD PROCESS DEFINITIONS
========================================================== */

async function loadProcesses() {

    const response = await fetch("/api/process", {
        cache: "no-store"
    });

    processes = await response.json();

    buildUI();

}

/* ==========================================================
   BUILD UI
========================================================== */

function buildUI() {

    container.innerHTML = "";

    for (const proc of processes) {

        const card = document.createElement("section");

        card.className = "process-card";

        card.innerHTML = `

<div class="card-header">

    <div>

        <h2>${proc.title}</h2>

        <p>${proc.command}</p>

    </div>

    <div class="controls">

        <span
            id="${proc.id}-status"
            class="status ${proc.running ? "running" : "stopped"}">

            ${proc.running ? "RUNNING" : "STOPPED"}

        </span>

        <button onclick="startProcess('${proc.id}')">
            ▶ Start
        </button>

        <button onclick="stopProcess('${proc.id}')">
            ■ Stop
        </button>

        <button onclick="clearProcess('${proc.id}')">
            Clear
        </button>

    </div>

</div>

<pre
    id="${proc.id}-output"
    class="terminal">
</pre>

${proc.interactive ? `

<div class="input-bar">

    <input
        id="${proc.id}-input"
        class="live-input"
        type="text"
        autocomplete="off"
        spellcheck="false"
        placeholder="Click here then press I J K L Space Q">

</div>

` : ""}

`;

        container.appendChild(card);
        
        if(proc.interactive){

            bindLiveInput(proc.id);

        }

    }

}

/* ==========================================================
   START
========================================================== */

async function startProcess(id) {

    await fetch(`/api/process/${id}/start`, {
        method:"POST"
    });

    await refresh();

}

/* ==========================================================
   STOP
========================================================== */

async function stopProcess(id) {

    await fetch(`/api/process/${id}/stop`, {
        method:"POST"
    });

    await refresh();

}

/* ==========================================================
   CLEAR
========================================================== */

async function clearProcess(id) {

    await fetch(`/api/process/${id}/clear`, {
        method:"POST"
    });

    document.getElementById(
        `${id}-output`
    ).textContent="";

}

/* ==========================================================
   LIVE INPUT
========================================================== */

function bindLiveInput(id){

    const input =
        document.getElementById(`${id}-input`);

    if(!input)
        return;

    input.addEventListener("keydown", async (e)=>{

        e.preventDefault();

        let key = e.key.toLowerCase();

        if(key === " ")
            key = " ";

        const allowed = [
            "i",
            "j",
            "k",
            "l",
            "q",
            " "
        ];

        if(!allowed.includes(key))
            return;

        await fetch(`/api/process/${id}/input`,{

            method:"POST",

            headers:{
                "Content-Type":"application/json"
            },

            body:JSON.stringify({
                input:key
            })

        });

    });

}

/* ==========================================================
   OUTPUT
========================================================== */

async function updateOutput(id){

    const response=await fetch(
        `/api/process/${id}/output`,
        {
            cache:"no-store"
        }
    );

    const lines=await response.json();

    const terminal=document.getElementById(
        `${id}-output`
    );

    if(!terminal)
        return;

    const text=lines.join("\n");

    if(terminal.textContent!==text){

        terminal.textContent=text;

        terminal.scrollTop=
            terminal.scrollHeight;

    }

}

/* ==========================================================
   STATUS
========================================================== */

async function updateStatus(){

    const response=await fetch(
        "/api/process",
        {
            cache:"no-store"
        }
    );

    const latest=await response.json();

    processes=latest;

    for(const proc of latest){

        const badge=document.getElementById(
            `${proc.id}-status`
        );

        if(!badge)
            continue;

        badge.className=
            `status ${proc.running ? "running":"stopped"}`;

        badge.textContent=
            proc.running
            ? "RUNNING"
            : "STOPPED";

    }

}

/* ==========================================================
   REFRESH
========================================================== */

async function refresh(){

    await updateStatus();

    for(const proc of processes){

        await updateOutput(proc.id);

    }

}

/* ==========================================================
   INITIALIZE
========================================================== */

(async()=>{

    await loadProcesses();

    await refresh();

    setInterval(refresh,1000);

})();
