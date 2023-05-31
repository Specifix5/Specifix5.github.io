function mainSetup() {
    $.getJSON("https://api.ipify.org?format=json", function(data) {
         
        document.getElementById("ipText").innerHTML = "Hello there, " + `<span style="font-family: 'Hack'">${data.ip}</span>!`;
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

}
