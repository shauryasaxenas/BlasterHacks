console.log("Script is working");

fetch("data.json")
.then(response => response.json())
.then(sections => {
    let placeholderAssignment = document.querySelector("#assignment-output");
    let placeholderGrades = document.querySelector("#grade-output");
    let placeholderGroq = document.querySelector("#groq-output");
    let outAssignment = "", outGrade = "", outGroq = "";

    // Loop through assignments
    sections.assignments.forEach(assignment => {
        const dueDateStr = assignment.date;
        const dueDate = new Date(dueDateStr);
        const today = new Date();
        today.setHours(0, 0, 0, 0); // Normalize today's date

        const timeDiff = dueDate - today;
        const daysRemaining = Math.ceil(timeDiff / (1000 * 60 * 60 * 24)); // Convert to days

        // Determine color gradient from Green (14+ days) → Yellow (6 days) → Red (0 days)
        let color;
        if (daysRemaining <= 0) {
            color = "rgb(139, 0, 0)"; // Dark red for due today or overdue
        } else if (daysRemaining <= 3) {
            let redIntensity = 255;
            let greenIntensity = Math.round(50 * (daysRemaining / 3)); // Darker red as daysRemaining decreases
            color = `rgb(${redIntensity}, ${greenIntensity}, 0)`;
        } else if (daysRemaining <= 6) {
            let redIntensity = 255;
            let greenIntensity = Math.round(200 - (100 * (6 - daysRemaining) / 3)); // Darker yellow as it gets closer
            color = `rgb(${redIntensity}, ${greenIntensity}, 0)`;
        } else if (daysRemaining <= 14) {
            let redIntensity = Math.round(255 * (14 - daysRemaining) / 8); // Green fades towards yellow
            let greenIntensity = 200;
            color = `rgb(${redIntensity}, ${greenIntensity}, 0)`;
        } else {
            color = "rgb(0, 200, 0)"; // Bright green for 14+ days away
        }

        // Format date for display
        let month = dueDateStr.slice(5, 7);
        let day = dueDateStr.slice(8, 10);
        let year = dueDateStr.slice(2, 4);
        let timeHour = parseInt(dueDateStr.slice(11, 13));
        let timeMin = dueDateStr.slice(14, 16);
        let timeOfDay = "AM";

        if (timeHour > 11) {
            if (timeHour !== 12) timeHour -= 12;
            timeOfDay = "PM";
        }

        let formattedDueDate = `${month}/${day}/${year} @ ${timeHour}:${timeMin} ${timeOfDay}`;
        let fullAssignment = `${assignment.course} - ${assignment.name}`;

        // Additional details
        const additionalInfo = `
            <div class="additional-info">
                <p>Description: ${assignment.description || "No description available"}</p>
                <p>
                    <a href="${assignment.html_url}" target="_blank">Canvas Link</a>
                </p>
            </div>
        `;

        // Assignment row with color-changing button
        outAssignment += `
            <tr class="assignment-row">
                <td>
                    <button class="assignment-toggle" style="background-color: ${color};">
                        <span style="display: inline-block; text-align: left; width: 76%;">${fullAssignment}</span>
                        <span style="display: inline-block; text-align: right;">${formattedDueDate}</span>
                    </button>
                    <div class="additional-details" style="display: none;">
                        ${additionalInfo}
                    </div>
                </td>
            </tr>
        `;
    });

    // Populate Grades Table
    sections.grades.forEach(grade => {
        let courseGrade = `${grade.course}: ${grade.grade}%`;
        let courseLink = `https://elearning.mines.edu/courses/${grade.course_id}/grades`;
        outGrade += `
            <tr>
                <td>
                    <a href="${courseLink}" target="_blank">${courseGrade}</a>
                </td>
            </tr>
        `;
    });

    // Populate Groq AI Table
    outGroq += `
        <tr>
            <td>${sections.plan}</td>
        </tr>
    `;

    // Insert content into the HTML
    placeholderAssignment.innerHTML = outAssignment;
    placeholderGrades.innerHTML = outGrade;
    placeholderGroq.innerHTML = outGroq;

    // Add event listeners for assignment toggles
    document.querySelectorAll('.assignment-toggle').forEach(button => {
        button.addEventListener('click', function() {
            const details = button.nextElementSibling;
            const row = button.closest('tr');
            const isVisible = details.style.display === "block";

            details.style.display = isVisible ? "none" : "block";
            row.classList.toggle('expanded');
        });
    });
});
