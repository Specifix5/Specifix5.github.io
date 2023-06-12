$(document).ready(function() {
    $.getJSON("https://api.ipify.org?format=json", function(data) {
        document.getElementById("ipText").innerHTML = '<span style="font-size:20px;">Hello ' + `<span style="font-family: 'Fira Code';border-right: 0.65em solid #eeeeee;animation: blink-caret 1s step-end infinite;">${data.ip}</span></span>`;
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
    } else {
        document.getElementById("avatar_img").style.marginLeft = "75px";
        document.getElementById("project-text-wrapper").style.width = "35%";
    }
});
