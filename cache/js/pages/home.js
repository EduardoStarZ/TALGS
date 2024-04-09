$(document).ready(function() {

    const dialog = document.querySelector("dialog");

    $("#login").click(function() {
        dialog.showModal();
        
    });

    $("#topo").click(function () {
        $("html, body").animate({ scrollTop: 0 }, 1000);
    });
})
