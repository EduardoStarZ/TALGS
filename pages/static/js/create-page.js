$(".has-consequences").click(function(e){
    let element_id = $(this).attr('id');

    let splitter = element_id.split('-')

    let buffer = splitter[0];

    switch(buffer) {
        case "available": {
            $(`#selected-card-${splitter[2]}`).removeClass("hidden");
            $(`#available-card-${splitter[2]}`).addClass("hidden");
        }
        break;
        case "selected": {
            $(`#selected-card-${splitter[2]}`).addClass("hidden");
            $(`#available-card-${splitter[2]}`).removeClass("hidden");
        }
        break;
    }
})