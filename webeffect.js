let messageboxwrapper = document.getElementById("messageboxwrapper")
let messageboxarea = document.getElementById("messageboxdet")

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



var handler = onVisibilityChange(messageboxarea, function(visible) {
    console.log(visible)
    if (visible) {
        messageboxwrapper.animate([{transform: "translateY(50px)", opacity: "0"}, {transform: "translateY(0px)", opacity: "1"}], {duration: 300, iterations: 1, });
    } else {
        //messageboxwrapper.animate([{transform: "translateY(0px)", opacity: "1"}, {transform: "translateY(50px)", opacity: "0"}], {duration: 300, iterations:1, });
    }
});

$(window).on('DOMContentLoaded load resize scroll', handler);