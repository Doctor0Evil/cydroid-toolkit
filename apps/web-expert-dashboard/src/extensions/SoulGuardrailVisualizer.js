/**
 * Soul Guardrail & NEU Budget Visualizer
 * 
 * Extension for the Cydroid Expert Dashboard.
 * Visualizes:
 * - NEU Psych-Risk Budget status (Green/Yellow/Red).
 * - Soul Guardrail Violation alerts (Red Flash).
 * - CyberRank Vector components (Radar Chart).
 * 
 * Compliance: Does NOT display soul scores (forbidden by soul.guardrail.spec.v1).
 * Only displays action/particle ranks and budget states.
 */

class SoulGuardrailVisualizer {
    constructor(containerId) {
        this.container = document.getElementById(containerId);
        this.neuBudgetElement = null;
        this.violationAlertElement = null;
        this.init();
    }

    init() {
        this.container.innerHTML = `
            <div id="neu-budget-container" style="padding: 10px; border: 1px solid #ccc;">
                <h3>NEU Psych-Risk Budget</h3>
                <div id="neu-bar" style="width: 100%; height: 20px; background: green;"></div>
                <p id="neu-text">100%</p>
            </div>
            <div id="soul-guardrail-alert" style="padding: 10px; border: 1px solid #ccc; margin-top: 10px;">
                <h3>Soul Guardrail Status</h3>
                <p id="soul-status" style="color: green;">COMPLIANT</p>
            </div>
            <canvas id="cyberrank-chart" width="400" height="400"></canvas>
        `;
        this.neuBudgetElement = document.getElementById('neu-bar');
        this.neuTextElement = document.getElementById('neu-text');
        this.violationAlertElement = document.getElementById('soul-status');
    }

    /**
     * Update NEU Budget Visualization
     * @param {number} current - Current NEU balance
     * @param {number} max - Max NEU balance
     */
    updateNeuBudget(current, max) {
        const percentage = (current / max) * 100;
        this.neuBudgetElement.style.width = `${percentage}%`;
        this.neuTextElement.innerText = `${percentage.toFixed(2)}%`;

        // Color coding based on exhaustion threshold (e.g., 20%)
        if (percentage < 20) {
            this.neuBudgetElement.style.background = 'red';
        } else if (percentage < 50) {
            this.neuBudgetElement.style.background = 'orange';
        } else {
            this.neuBudgetElement.style.background = 'green';
        }
    }

    /**
     * Alert on Soul Guardrail Violation
     * @param {boolean} violated - True if violation detected
     */
    updateSoulGuardrailStatus(violated) {
        if (violated) {
            this.violationAlertElement.innerText = "VIOLATION DETECTED";
            this.violationAlertElement.style.color = "red";
            this.violationAlertElement.style.fontWeight = "bold";
            // Trigger audit log
            this.logViolation();
        } else {
            this.violationAlertElement.innerText = "COMPLIANT";
            this.violationAlertElement.style.color = "green";
            this.violationAlertElement.style.fontWeight = "normal";
        }
    }

    /**
     * Log Violation to Audit Trail
     */
    logViolation() {
        console.warn("Soul Guardrail Violation Logged at", new Date().toISOString());
        // In production: Send to ROW ledger via secure API
    }

    /**
     * Render CyberRank Vector (Radar Chart)
     * @param {object} vector - { safety, legal, biomech, psych, rollback }
     */
    renderCyberRank(vector) {
        // Placeholder for Chart.js implementation
        // Ensures 'soul' is NOT an axis (only action dimensions)
        console.log("Rendering CyberRank:", vector);
    }
}

// Initialize
const visualizer = new SoulGuardrailVisualizer('dashboard-extension-container');
