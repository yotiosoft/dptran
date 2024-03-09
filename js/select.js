function switch_to_windows() {
    let current_option = document.getElementsByClassName("os_select is_active")[0];
    if (current_option != null)
        current_option.classList.remove("is_active");

    let selected_option = document.getElementById("select_windows");
    selected_option.classList.add("is_active");

    let current_view = document.getElementsByClassName("os_document is_active")[0];
    if (current_view != null)
        current_view.classList.remove("is_active");

    let selected_view = document.getElementById("switch_view_windows");
    selected_view.classList.add("is_active");

    let current_imgs = Array.prototype.slice.call(document.getElementsByClassName("os_img is_active"));
    if (current_imgs != null) {
        for (var current_img of current_imgs) {
            current_img.classList.remove("is_active");
        }
    }

    let selected_imgs = Array.prototype.slice.call(document.getElementsByClassName("os_img img_windows"));
    if (selected_imgs != null) {
        for (var selected_img of selected_imgs) {
            selected_img.classList.add("is_active");
        }
    }
}

function switch_to_macos() {
    let current_option = document.getElementsByClassName("os_select is_active")[0];
    if (current_option != null)
        current_option.classList.remove("is_active");

    let selected_option = document.getElementById("select_macos");
    selected_option.classList.add("is_active");

    let current_view = document.getElementsByClassName("os_document is_active")[0];
    if (current_view != null)
        current_view.classList.remove("is_active");

    let selected_view = document.getElementById("switch_view_macos");
    selected_view.classList.add("is_active");

    let current_imgs = Array.prototype.slice.call(document.getElementsByClassName("os_img is_active"));
    if (current_imgs != null) {
        for (var current_img of current_imgs) {
            current_img.classList.remove("is_active");
        }
    }

    let selected_imgs = Array.prototype.slice.call(document.getElementsByClassName("os_img img_macos"));
    if (selected_imgs != null) {
        for (var selected_img of selected_imgs) {
            selected_img.classList.add("is_active");
        }
    }
}

function switch_to_linux() {
    let current_option = document.getElementsByClassName("os_select is_active")[0];
    if (current_option != null)
        current_option.classList.remove("is_active");

    let selected_option = document.getElementById("select_linux");
    selected_option.classList.add("is_active");

    let current_view = document.getElementsByClassName("os_document is_active")[0];
    if (current_view != null)
        current_view.classList.remove("is_active");

    let selected_view = document.getElementById("switch_view_linux");
    selected_view.classList.add("is_active");

    let current_imgs = Array.prototype.slice.call(document.getElementsByClassName("os_img is_active"));
    if (current_imgs != null) {
        for (var current_img of current_imgs) {
            console.log(current_img);
            current_img.classList.remove("is_active");
        }
    }

    let selected_imgs = Array.prototype.slice.call(document.getElementsByClassName("os_img img_linux"));
    if (selected_imgs != null) {
        for (var selected_img of selected_imgs) {
            selected_img.classList.add("is_active");
        }
    }
}

window.onload = function() {
    if (navigator.userAgent.indexOf("Win") != -1) {
        switch_to_windows();
    }
    else if (navigator.userAgent.indexOf("Mac") != -1) {
        switch_to_macos();
    }
    else if (navigator.userAgent.indexOf("Linux") != -1) {
        switch_to_linux();
    }
    else {
        switch_to_windows();
    }
}
