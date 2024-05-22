let sendMessageDEFAULT = `Message me anonymously!`

function debounce(method, delay) {
    clearTimeout(method._tId);
    method._tId= setTimeout(function(){
        method();
    }, delay);
}

function xMouseOver() {
    var element = document.querySelector('.social-twitter > i')
    element.classList.remove('fa-twitter')
    element.classList.add('fa-x-twitter')
}

function xMouseOut() {
    var element = document.querySelector('.social-twitter > i')
    element.classList.remove('fa-x-twitter')
    element.classList.add('fa-twitter')
}

function sendMessage(messagebox) {
    var post = $.ajax({
        type: "POST",
        url: "https://api.specifix.dev/api/mailbox",
        data: JSON.stringify({ "content": messagebox.value }),
        contentType: "application/json",
        success: function(response) {
            messagebox.value = ""
            messagebox.placeholder = "Sent!"
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

$(document).ready(function() { 
    document.getElementById("year").innerText = `${(new Date()).getFullYear() - 2018} years`

    if (window.innerWidth > 860) {
        header.style.backgroundImage = "linear-gradient(180deg, #0f141d, #0f141d00)"
        header.style.boxShadow = "0px 0px"
    }

    let hour = Number(new Date().toLocaleString('en-US', {hour12: false}).split(", ")[1].split(":")[0]);

    if (hour < 18) {
        if (hour < 1) {
            document.getElementById("goodday").innerText = `Evening`;
        } else if(hour > 11) {
            document.getElementById("goodday").innerText = `Afternoon`;
        } else {
            document.getElementById("goodday").innerText = `Morning`;
        }
    } else {
        document.getElementById("goodday").innerText = `Evening`;
    }

    try {
        $.getJSON("https://api.specifix.dev/api/ip", function(data) {
            document.getElementById("ip").textContent = ` ${data.ip}`;
        });

        $.getJSON("https://api.specifix.dev/api/subcount", function(data) {
            document.getElementById("ytcount").innerText = `${data.subCount}`;
        })
    } catch (err) {
        console.log(err);
    }

    let sendButton = document.getElementById("sendmessage")
    let messagebox = document.getElementById("messagebox")

    sendButton.addEventListener("click", function() {
        sendMessage(messagebox)
    });
    
    var element = document.getElementById("mainpagecontainer")
    element.addEventListener('scroll', event => {
        debounce(() => {
            var header = document.getElementById("header")
            var logotext = document.getElementById("logotext");
            var l_iptext = document.getElementById("logoiptext");
            if (window.innerWidth > 900) {
                var scroll = element.scrollTop/(window.innerHeight)//element.scrollTop/(element.scrollHeight - window.innerHeight)
                if (scroll > 0.85) { // SHOW
                    header.style.backgroundColor = "#151d29"
                    header.style.boxShadow = "3px 3px 3px #0c0f14b2"
                    logotext.classList.remove("centerlogotext")
                    l_iptext.classList.remove("centerlogotextremove")
                } else {
                    header.style.backgroundColor = "#151d2900"
                    header.style.boxShadow = "0px 0px"
                    logotext.classList.add("centerlogotext")
                    l_iptext.classList.add("centerlogotextremove")
                }
            } else { // SHOW
                header.style.backgroundColor = "#151d29"
                header.style.boxShadow = "3px 3px 3px #0c0f14b2"
                logotext.classList.remove("centerlogotext")
                l_iptext.classList.remove("centerlogotextremove")
            } 
        }, 100)
    });
});

