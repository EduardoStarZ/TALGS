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
				let splitter = element_id.split('-')
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
						}
								break;
				}
		})

})

$("#cancel").click(function() {
		$("#htmx-target").empty();
		$(".has-consequences").removeClass('hidden');
})
