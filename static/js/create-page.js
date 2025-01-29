/*
 *
 * create-page.js
 *
 * Copyright (c) 2023-2024 EduardoStarZ, NandoBFK, Erenan257
 *
 * All rights reserved
 *
 * TALGS is distributed under the GNU General Public license v2, see LICENSE for details
 * 
 * */

// Dinamically loads every component that has a 'has-consequences' class into the event listener
htmx.onLoad(function(content) {
		// Creates an event listener to allow for click listening
		$(".has-consequences").click(function(e){
				let element_id = $(this).attr('id');
				let splitter = element_id.split('+');
				let buffer = splitter[0];
				
				console.log(`${element_id}, ${splitter}`);

				let available = document.getElementById("available+" + splitter[2]);
				let selected = document.getElementById("selected+" + splitter[2]);

				switch(buffer) {
						case "available": {
								available.classList.add("hidden");
								//$(`#selected+${splitter[2]}`).removeClass("hidden");
								//$('#available+' + splitter[2]).addClass("hidden");
						}
								break;
						case "selected": {

								available.classList.remove("hidden");
								selected.parentNode.removeChild(selected);

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

		htmx.ajax('GET', '/api/product/available_card?category='+id, '#htmx-available');
})


// Refreshes the elements supposed to appear on the available frame on the create page, only showing the elements that aren't on the
// selected frame
function check_for_frame_changes() {

		elements = document.querySelectorAll("#htmx-target .frame");
		
		id = $("#filtro").children("option:selected").val();

		let target = `/api/product/available_card?category=${id}`;

		if (elements.len > 0) {
				target += '&exclude';
		}

		elements.forEach((value) => {
				target += value.id.replace('selected+', '') + ",";
		})

		htmx.ajax('GET', target, '#htmx-available');
}

// event listener for the select element
htmx.on('#filtro', 'change', check_for_frame_changes);
