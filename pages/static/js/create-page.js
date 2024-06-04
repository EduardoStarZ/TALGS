/*
 *
 * craete-page.js
 *
 * Copyright (c) 2023-2024 (authors)
 *
 * All rights reserved
 *
 * TALGS is distributed under the () license, see LICENSE for details
 * 
 * */


// Dinamically loads every component that has a 'has-consequences' class into the event listener
htmx.onLoad(function(content) {
		// Creates an event listener to allow for click listening
		$(".has-consequences").click(function(e){
				let element_id = $(this).attr('id');
				let splitter = element_id.split('-');
				let buffer = splitter[0];

				switch(buffer) {
						case "available": {
								$(`#selected-card-${splitter[2]}`).removeClass("hidden");
								$(`#available-card-${splitter[2]}`).addClass("hidden");
						}
								break;
						case "selected": {
								$(`#available-card-${splitter[2]}`).removeClass("hidden");
								$(`#selected-card-${splitter[2]}`).remove();
								$(`#product-id-${splitter[2]}`).remove();
								$(`#product-amount-${splitter[2]}`).remove();
								check_for_frame_changes();
						}
								break;
				}
		})

})


// Clears the whole selected frame
$("#cancel").click(function() {

		$("#htmx-target").empty();
		$("htmx-available").empty();

		let id = $('#filtro').children("option:selected").val();

		htmx.ajax('GET', '/api/sale/available_card?filtro='+id, '#htmx-available');
})


// Refreshes the elements supposed to appear on the available frame on the create page, only showing the elements that aren't on the
// selected frame
function check_for_frame_changes() {

		elements = document.querySelectorAll("#htmx-target .frame");
		
		id = $(event.target).children("option:selected").val();

		let target = `/api/sale/available_card?filtro=${id}`;

		elements.forEach((value) => {
				target += '&exclude=' + value.id.replace('selected-card-', '')
		})

		htmx.ajax('GET', target, '#htmx-available');
}

// event listener for the select element
htmx.on('#filtro', 'change', check_for_frame_changes);
