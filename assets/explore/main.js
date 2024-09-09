const dropdown = document.getElementById("dropdown");
const optionsContainer = document.getElementById("optionsContainer");
const selectedOptions = document.getElementById("selectedOptions");

let selectedValues = [];

// Toggle options container visibility
dropdown.addEventListener("click", () => {
	optionsContainer.style.display =
		optionsContainer.style.display === "block" ? "none" : "block";
});

// Handle selection of options
optionsContainer.addEventListener("change", (event) => {
	const optionValue = event.target.value;

	if (event.target.checked) {
		selectedValues.push(optionValue);
	} else {
		selectedValues = selectedValues.filter((value) => value !== optionValue);
	}

	// Update the input field with selected options
	selectedOptions.value = selectedValues.join(", ");
});

// Close the options container when clicking outside
document.addEventListener("click", (event) => {
	if (!dropdown.contains(event.target)) {
		optionsContainer.style.display = "none";
	}
});
