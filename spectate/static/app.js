let lastToolsJson = "";

async function loadTools(force = false) {
    try {

        const response = await fetch("/api/tools", {
            cache: "no-store"
        });

        if (!response.ok) {
            throw new Error(`HTTP ${response.status}`);
        }

        const tools = await response.json();

        const json = JSON.stringify(tools);

        if (!force && json === lastToolsJson) {
            return;
        }

        lastToolsJson = json;

        renderTools(tools);

    } catch (err) {
        console.error("Failed to load tools:", err);
    }
}

function renderTools(tools) {

    const container = document.getElementById("tools-grid");

    container.innerHTML = "";

    if (tools.length === 0) {

        container.innerHTML = `
            <div class="tool-card empty-card">

                <div class="card-header">

                    <h3>No Tools</h3>

                    <span class="status-bad">
                        EMPTY
                    </span>

                </div>

                <div class="card-body">
                    No tool links were discovered.
                </div>

            </div>
        `;

        return;
    }

    for (const tool of tools) {

        const group =
            tool.group && tool.group.length > 0
                ? tool.group
                : "root";

        const healthy = tool.status === "Healthy";

        const card = document.createElement("div");
        card.className = "tool-card";

        card.innerHTML = `

<div class="card-header">

    <div>

        <h3>${tool.name}</h3>

        <div class="group">
            ${group}
        </div>

    </div>

    <span class="${healthy ? "status-ok" : "status-bad"}">
        ${healthy ? "HEALTHY" : "BROKEN"}
    </span>

</div>

<div class="card-body">

    <div class="meta">

        <span class="meta-label">
            SOURCE
        </span>

        <code>
${tool.source}
        </code>

    </div>

    <div class="meta">

        <span class="meta-label">
            TARGET
        </span>

        <code>
${tool.target}
        </code>

    </div>

</div>

<div class="card-footer">

    <div class="footer-left">

        <span class="dot ${healthy ? "dot-green" : "dot-red"}"></span>

        ${healthy ? "Symlink OK" : "Broken Link"}

    </div>

</div>

`;

        container.appendChild(card);
    }
}

async function refreshTools() {
    await loadTools(true);
}

const refreshButton = document.getElementById("refresh-tools");

if (refreshButton) {
    refreshButton.addEventListener("click", refreshTools);
}

loadTools(true);

setInterval(() => {
    loadTools(false);
}, 60000);
