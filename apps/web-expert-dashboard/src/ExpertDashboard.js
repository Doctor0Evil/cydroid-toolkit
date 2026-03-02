/**
 * Cydroid Expert Dashboard
 * 
 * Remote interface for scientists and auditors to view:
 * - ROW Ledger Integrity
 * - Eco-Metric Aggregations
 * - Evidence Bundle Verification
 * - Consent Audit Trails
 * 
 * Does NOT expose raw biophysical streams (Privacy/Neurorights).
 * Connects to Cydroid ALN Gateway via WebSocket.
 */

class ExpertDashboard {
    constructor(wsUrl) {
        this.wsUrl = wsUrl;
        this.ws = null;
        this.ledger = [];
        this.ecoMetrics = [];
    }

    connect() {
        this.ws = new WebSocket(this.wsUrl);
        this.ws.onmessage = (event) => this.handleMessage(JSON.parse(event.data));
        this.ws.onopen = () => console.log("Connected to Cydroid ALN Gateway");
    }

    handleMessage(msg) {
        if (msg.type === "ROW_ENTRY") {
            this.ledger.push(msg.payload);
            this.renderLedger();
        } else if (msg.type === "ECO_METRIC") {
            this.ecoMetrics.push(msg.payload);
            this.renderEcoChart();
        }
    }

    // Verify ROW Chain Integrity (SHA-256 Hash Linking)
    verifyChainIntegrity() {
        for (let i = 1; i < this.ledger.length; i++) {
            const prev = this.ledger[i - 1];
            const curr = this.ledger[i];
            const hash = this.computeSHA256(JSON.stringify(prev));
            if (curr.previous_entry_hash !== hash) {
                console.error("ROW Chain Integrity Violation at index", i);
                return false;
            }
        }
        return true;
    }

    computeSHA256 async (message) {
        const msgBuffer = new TextEncoder().encode(message);
        const hashBuffer = await crypto.subtle.digest('SHA-256', msgBuffer);
        const hashArray = Array.from(new Uint8Array(hashBuffer));
        return hashArray.map(b => b.toString(16).padStart(2, '0')).join('');
    }

    renderLedger() {
        const container = document.getElementById('row-ledger');
        container.innerHTML = '';
        // Display last 10 entries
        this.ledger.slice(-10).reverse().forEach(entry => {
            const div = document.createElement('div');
            div.className = 'ledger-entry';
            div.innerHTML = `
                <strong>${entry.entry_type}</strong> | 
                DID: ${entry.signer_did.substring(0, 20)}... | 
                Time: ${new Date(entry.timestamp).toISOString()}
            `;
            container.appendChild(div);
        });
    }

    renderEcoChart() {
        // Placeholder for Chart.js or D3.js integration
        const avgScore = this.ecoMetrics.reduce((a, b) => a + b.value, 0) / this.ecoMetrics.length;
        document.getElementById('eco-score-display').innerText = avgScore.toFixed(3);
        
        // Visual Warning if below floor
        const indicator = document.getElementById('eco-floor-indicator');
        if (avgScore < 0.86) {
            indicator.style.backgroundColor = 'red';
            indicator.innerText = "BELOW FLOOR";
        } else {
            indicator.style.backgroundColor = 'green';
            indicator.innerText = "COMPLIANT";
        }
    }
}

// Initialize
const dashboard = new ExpertDashboard('wss://cydroid-gateway.local/aln');
dashboard.connect();
