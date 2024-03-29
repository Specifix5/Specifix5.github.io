const pause = (time) => new Promise(resolve => setTimeout(resolve, time))

var confettionlyOnce = false

async function loadButtons(sbtn) {
    for (var i=0;i < sbtn.length;i++) {
        sbtn.item(i).style.animationPlayState = "running";
        await pause(300);
    }
}

function onVisibilityChange(el, callback) {
    var old_visible;
    return function () {
        var visible = isElementInViewport(el);
        if (visible != old_visible) {
            old_visible = visible;
            if (typeof callback == 'function') {
                callback(visible);
            }
        }
    }
}

function isElementInViewport (el) {
    if (typeof jQuery === "function" && el instanceof jQuery) {
        el = el[0];
    }

    var rect = el.getBoundingClientRect();

    return (
        rect.top >= 0 &&
        rect.left >= 0 &&
        rect.bottom <= (window.innerHeight || document.documentElement.clientHeight) && /* or $(window).height() */
        rect.right <= (window.innerWidth || document.documentElement.clientWidth) /* or $(window).width() */
    );
}

let messageboxwrapper = document.getElementById("messageboxwrapper")
let messageboxarea = document.getElementById("messageboxdet")

var thxtext = document.getElementById("thxtext")

var sbtn = document.getElementsByClassName("socialbutton")
loadButtons(sbtn);



var handler = onVisibilityChange(messageboxarea, function(visible) {
    //console.log(visible)
    if (visible) {
        messageboxwrapper.animate([{transform: "translateY(50px)", opacity: "0"}, {transform: "translateY(0px)", opacity: "1"}], {duration: 300, iterations: 1, });
    } else {
        messageboxwrapper.animate([{transform: "translateY(0px)", opacity: "1"}, {transform: "translateY(50px)", opacity: "0"}], {duration: 300, iterations:1, });
    }
});

var particleCount = 15
if (navigator.userAgent.match(/Android/i)
        || navigator.userAgent.match(/webOS/i)
        || navigator.userAgent.match(/iPhone/i)
        || navigator.userAgent.match(/iPad/i)
        || navigator.userAgent.match(/iPod/i)
        || navigator.userAgent.match(/BlackBerry/i)
        || navigator.userAgent.match(/Windows Phone/i)) {
            particleCount = 15
    } else {
        particleCount = 60
    }

var handler2 = onVisibilityChange(thxtext, function(visible) {
    if (visible && confettionlyOnce != true) {
        confettionlyOnce = true
        confetti({
            particleCount: particleCount,
            angle: 60,
            spread: 145,
            origin: { x: 0 }
          });
        confetti({
            particleCount: particleCount,
            angle: 120,
            spread: 145,
            origin: { x: 1 }
        });
    }
})

$(window).on('DOMContentLoaded load resize scroll', handler);
$(window).on('DOMContentLoaded load resize scroll', handler2);