let sendMessageDEFAULT = `Send me a message anonymously`

$(document).ready(function() {
    try {
        $.getJSON("https://api.specifix.dev/api/ip", function(data) {
            document.getElementById("ipText").innerText = ` ${data.ip}`;
        });

    } catch (err) {
        console.log(err);
    }

    let hour = Number(new Date().toLocaleString('en-US', {hour12: false}).split(", ")[1].split(":")[0]);
    if (hour < 18) {
        if (hour > 5) {
            document.getElementById("goodday").innerText = `have a good day!`;
        } else {
            document.getElementById("goodday").innerText = `have a good night!`;
        }
    } else {
        document.getElementById("goodday").innerText = `have a good night!`;
    }

    let sendButton = document.getElementById("sendButton")
    let messagebox = document.getElementById("message")

    sendButton.addEventListener("click", function() {
        sendMessage(messagebox)
    });
    
    document.getElementById("prYear").innerText = `${(new Date()).getFullYear() - 2018} years`
    if (navigator.userAgent.match(/Android/i)
        || navigator.userAgent.match(/webOS/i)
        || navigator.userAgent.match(/iPhone/i)
        || navigator.userAgent.match(/iPad/i)
        || navigator.userAgent.match(/iPod/i)
        || navigator.userAgent.match(/BlackBerry/i)
        || navigator.userAgent.match(/Windows Phone/i)) {
            document.getElementById("avatar_img").style.marginLeft = "10px";
            document.getElementById("project-text-wrapper").style.width = "100%";
            sendMessageDEFAULT = `Send me a message`
            messagebox.placeholder = sendMessageDEFAULT
    } else {
        document.getElementById("avatar_img").style.marginLeft = "75px";
        document.getElementById("project-text-wrapper").style.width = "35%";
    }
});


function sendMessage(messagebox) {
    var post = $.ajax({
        type: "POST",
        url: "https://api.specifix.dev/api/mailbox",
        data: JSON.stringify({ "content": messagebox.value }),
        contentType: "application/json",
        success: function(response) {
            messagebox.value = ""
            messagebox.placeholder = "Done!"
            setTimeout(function() {
                messagebox.placeholder = sendMessageDEFAULT
            }, 1500)
        },
        error: function (xhr, ajaxOptions, thrownError) {
            messagebox.value = ""
            let errmsg = `${thrownError}`
            if (xhr.responseText.length && xhr.responseText.length >= 2) {
                errmsg = xhr.responseText
            }
            messagebox.placeholder = `${xhr.status}: ${errmsg}`
            messagebox.classList.add("errorplaceholder")
            setTimeout(function() {
                messagebox.placeholder = sendMessageDEFAULT
                messagebox.classList.remove("errorplaceholder")
            }, 5500)
        }
    })
}