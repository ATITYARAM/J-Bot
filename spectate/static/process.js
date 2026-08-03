const processes = ["teleop", "build", "s3"];

/* ==========================================================
   START
========================================================== */

async function start(name) {

    await fetch(`/api/process/${name}/start`, {
        method: "POST"
    });

    updateStatus();
}

/* ==========================================================
   STOP
========================================================== */

async function stop(name) {

    await fetch(`/api/process/${name}/stop`, {
        method: "POST"
    });

    updateStatus();
}

/* ==========================================================
   CLEAR
========================================================== */

async function clearOutput(name) {

    await fetch(`/api/process/${name}/clear`, {
        method: "POST"
    });

    document.getElementById(`${name}-output`).textContent = "";
}

/* ==========================================================
   OUTPUT
========================================================== */

async function updateOutput(name) {

    try {

        const response =
            await fetch(`/api/process/${name}/output`, {
                cache: "no-store"
            });

        const lines = await response.json();

        const terminal =
            document.getElementById(`${name}-output`);

        const text = lines.join("\n");

        if (terminal.textContent !== text) {

            terminal.textContent = text;

            terminal.scrollTop =
                terminal.scrollHeight;

        }

    } catch (err) {

        console.error(err);

    }

}

/* ==========================================================
   STATUS
========================================================== */

async function updateStatus() {

    try {

        const response =
            await fetch("/api/process", {
                cache: "no-store"
            });

        const list = await response.json();

        for (const proc of list) {

            const badge =
                document.getElementById(
                    `${proc.name}-status`
                );

            if (!badge)
                continue;

            if (proc.running) {

                badge.className =
                    "status running";

                badge.textContent =
                    "RUNNING";

            } else {

                badge.className =
                    "status stopped";

                badge.textContent =
                    "STOPPED";

            }

        }

    } catch (err) {

        console.error(err);

    }

}

/* ==========================================================
   REFRESH
========================================================== */

async function refresh() {

    await updateStatus();

    for (const p of processes) {

        await updateOutput(p);

    }

}

/* ==========================================================
   EVENTS
========================================================== */

function bind(name) {

    document
        .getElementById(`${name}-start`)
        .addEventListener(
            "click",
            () => start(name)
        );

    document
        .getElementById(`${name}-stop`)
        .addEventListener(
            "click",
            () => stop(name)
        );

    document
        .getElementById(`${name}-clear`)
        .addEventListener(
            "click",
            () => clearOutput(name)
        );

}

for (const p of processes) {

    bind(p);

}

/* ==========================================================
   INITIAL LOAD
========================================================== */

refresh();

/* ==========================================================
   AUTO REFRESH
========================================================== */

setInterval(refresh, 1000);
