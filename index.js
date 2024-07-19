function showPopup(popup) {
    var popupElement = document.querySelector("#" + popup);
    if (popupElement != null) {
        var backdrop = document.querySelector("#popup-bg");
        backdrop.classList.add("visible");
        popupElement.classList.add("visible");
        
        var vhides = document.querySelectorAll(".vhide")
        vhides.forEach(element => { element.style.display = "none"; });

        document.querySelector("body").style.overflow = "hidden";

        window.scrollTo(0, 0);
    } else {
        console.error("Cannot find popup: " + popup)
    }
}

function hideAllPopups() {
    var backdrop = document.querySelector("#popup-bg");
    backdrop.classList.remove("visible");

    var popups = document.querySelectorAll(".popup");
    popups.forEach(element => {
        element.classList.remove("visible");
    });

    var vhides = document.querySelectorAll(".vhide");
    vhides.forEach(element => { element.style.display = "block"; });

    document.querySelector("body").style.overflow = "auto";
}

function sendbutton_visibility() {
    var input = document.querySelector("#sendPMInput");
    var button = document.querySelector("#sendPM");

    if (input.value.length > 0) {
        button.classList.add("visible");
    } else {
        button.classList.remove("visible");
    }
}

async function ajaxWrapper(_ajax) {
    return $.ajax (_ajax);
}

var debounce = false;

function sendbutton_validate() {
    var input = document.querySelector("#sendPMInput");

    try {
        if (input.value.length > 0 && !debounce) {
            debounce = true;
            var _value = input.value;
            var _xhr = "";
            input.value = "";
            input.placeholder = "> Sending message..."
            ajaxWrapper({
                type: "POST",
                url: "https://api.specifix.dev/api/mailbox",
                data: JSON.stringify({ "content": _value }),
                contentType: "application/json",
                success: function () {
                    input.placeholder = "> Successfully sent!"
                    debounce = false;
                    setTimeout(() => {
                        input.placeholder = "> send me a message anonymously"
                    }, 1500)
                },
                error: function (xhr, ajaxOptions, thrownError) {
                    _xhr = `${xhr.status} ${xhr.responseText}`

                    input.value = "";
                    input.placeholder = `> Error: ${_xhr}`;
                    setTimeout(() => {
                        input.placeholder = "> send me a message anonymously"
                        debounce = false;
                    }, 1500)
                }
            });
        } else {
            return
        }
    } catch (error) {
        debounce = true;
        input.value = "";
        input.placeholder = `> ${error}`;
        setTimeout(() => {
            input.placeholder = "> send me a message anonymously"
            debounce = false;
        }, 1500)
    } finally {
        return sendbutton_visibility();
    }
}

$(document).ready (()=> {
    setTimeout(() => {
        try { 
            document.body = twemoji.parse(document.body, { base: 'https://cdn.jsdelivr.net/gh/twitter/twemoji@14.0.2/assets/' });
        } catch (error) { console.log (error); }
    }, 50);

    try {
        let hour = Number(new Date().toLocaleString('en-US', {hour12: false}).split(", ")[1].split(":")[0]);
        let emoji = "☀️";
        let daynight = "day";

        if (hour < 18) {
            if (hour < 6) {
                emoji = `🌙`;
                daynight = "evening";
            }
        } else {
            emoji = `🌙`;
            daynight = "evening";
        }

        $.getJSON("https://api.specifix.dev/api/ip", function(data) {
            document.querySelector("#ip").textContent = ` ${data.ip}${emoji}`;
            document.querySelector("#daynight").textContent = daynight;
            document.querySelector("#year").innerHTML = `${(new Date()).getMonth() > 3 ? "more than" : "almost"} <span class="blue">${(new Date()).getFullYear() - 2018} years ago</span>`
            twemoji.parse(document.querySelector("#ip"), { base: 'https://cdn.jsdelivr.net/gh/twitter/twemoji@14.0.2/assets/' });
        });

        $.getJSON("https://api.specifix.dev/api/subcount", function(data) {
            document.getElementById("subcount").textContent = `${data.subCount} subs`;
        })
    } catch (error) { console.log(error); }
})