/**
 * CM Rust - Game Interface JavaScript
 * A Championship Manager 01/02-style football manager simulator
 */

// ============================================
// Game Data & State
// ============================================

const GameData = {
    clubs: [
        {
            id: "LIV",
            name: "Liverpool",
            short_name: "LIV",
            nation_id: "ENG",
            reputation: 92,
            balance: 50000000,
            transfer_budget: 30000000,
            wage_budget: 2000000,
            primary_color: "#C8102E",
            secondary_color: "#FFFFFF"
        },
        {
            id: "ARS",
            name: "Arsenal",
            short_name: "ARS",
            nation_id: "ENG",
            reputation: 90,
            balance: 45000000,
            transfer_budget: 25000000,
            wage_budget: 1800000,
            primary_color: "#EF0107",
            secondary_color: "#FFFFFF"
        },
        {
            id: "MUN",
            name: "Manchester United",
            short_name: "MUN",
            nation_id: "ENG",
            reputation: 95,
            balance: 80000000,
            transfer_budget: 50000000,
            wage_budget: 3000000,
            primary_color: "#DA291C",
            secondary_color: "#000000"
        },
        {
            id: "CHE",
            name: "Chelsea",
            short_name: "CHE",
            nation_id: "ENG",
            reputation: 88,
            balance: 60000000,
            transfer_budget: 40000000,
            wage_budget: 2500000,
            primary_color: "#034694",
            secondary_color: "#FFFFFF"
        }
    ],

    tactics: [
        { id: "442_balanced", name: "4-4-2 Balanced", formation: "442", mentality: "balanced", tempo: "normal", pressing: 50, width: 50 },
        { id: "433_attacking", name: "4-3-3 Attacking", formation: "433", mentality: "attacking", tempo: "high", pressing: 70, width: 60 },
        { id: "352_defensive", name: "3-5-2 Defensive", formation: "352", mentality: "defensive", tempo: "low", pressing: 30, width: 40 },
        { id: "442_counter", name: "4-4-2 Counter", formation: "442", mentality: "counter", tempo: "normal", pressing: 40, width: 45 }
    ],

    formations: {
        "442": [
            { row: 0, players: [{ pos: "GK", num: 1 }] },
            { row: 1, players: [{ pos: "RB", num: 2 }, { pos: "CB", num: 4 }, { pos: "CB", num: 5 }, { pos: "LB", num: 3 }] },
            { row: 2, players: [{ pos: "RM", num: 7 }, { pos: "CM", num: 8 }, { pos: "CM", num: 6 }, { pos: "LM", num: 11 }] },
            { row: 3, players: [{ pos: "ST", num: 9 }, { pos: "ST", num: 10 }] }
        ],
        "433": [
            { row: 0, players: [{ pos: "GK", num: 1 }] },
            { row: 1, players: [{ pos: "RB", num: 2 }, { pos: "CB", num: 4 }, { pos: "CB", num: 5 }, { pos: "LB", num: 3 }] },
            { row: 2, players: [{ pos: "CM", num: 8 }, { pos: "CDM", num: 6 }, { pos: "CM", num: 10 }] },
            { row: 3, players: [{ pos: "RW", num: 7 }, { pos: "ST", num: 9 }, { pos: "LW", num: 11 }] }
        ],
        "352": [
            { row: 0, players: [{ pos: "GK", num: 1 }] },
            { row: 1, players: [{ pos: "CB", num: 4 }, { pos: "CB", num: 5 }, { pos: "CB", num: 6 }] },
            { row: 2, players: [{ pos: "RM", num: 2 }, { pos: "CM", num: 8 }, { pos: "CDM", num: 4 }, { pos: "CM", num: 10 }, { pos: "LM", num: 3 }] },
            { row: 3, players: [{ pos: "ST", num: 9 }, { pos: "ST", num: 11 }] }
        ],
        "451": [
            { row: 0, players: [{ pos: "GK", num: 1 }] },
            { row: 1, players: [{ pos: "RB", num: 2 }, { pos: "CB", num: 4 }, { pos: "CB", num: 5 }, { pos: "LB", num: 3 }] },
            { row: 2, players: [{ pos: "RM", num: 7 }, { pos: "CM", num: 8 }, { pos: "CDM", num: 6 }, { pos: "CM", num: 10 }, { pos: "LM", num: 11 }] },
            { row: 3, players: [{ pos: "ST", num: 9 }] }
        ]
    },

    // Player names pool for generation
    playerNames: {
        first: ["James", "Marcus", "Lucas", "Gabriel", "Bruno", "Mason", "Jack", "Harry", "Phil", "John", "Kyle", "Declan", "Bukayo", "Erling", "Mohamed", "Virgil", "Trent", "Andrew", "Darwin", "Luis", "Raheem", "Kevin", "Rodri", "Bernardo", "Ruben"],
        last: ["Smith", "Johnson", "Williams", "Brown", "Jones", "Garcia", "Miller", "Davis", "Rodriguez", "Martinez", "Wilson", "Anderson", "Thomas", "Taylor", "Moore", "Jackson", "Martin", "Lee", "Perez", "Thompson", "White", "Harris", "Sanchez", "Clark", "Ramirez"]
    },

    nationalities: ["🏴󠁧󠁢󠁥󠁮󠁧󠁿", "🇧🇷", "🇫🇷", "🇪🇸", "🇩🇪", "🇵🇹", "🇳🇱", "🇧🇪", "🇦🇷", "🇳🇴"],

    positions: ["GK", "DEF", "DEF", "DEF", "DEF", "MID", "MID", "MID", "MID", "ATT", "ATT"]
};

// Game State
let GameState = {
    manager: "",
    club: null,
    currentDate: new Date(2024, 7, 1), // August 1, 2024
    season: "2024/25",
    formation: "442",
    mentality: 3,
    pressing: 50,
    tempo: "normal",
    squad: [],
    leagueTable: [],
    fixtures: [],
    currentFixtureIndex: 0,
    results: [],
    news: [],
    form: [],
    transferTargets: []
};

// ============================================
// Utility Functions
// ============================================

function formatCurrency(amount) {
    if (amount >= 1000000) {
        return `£${(amount / 1000000).toFixed(1)}M`;
    } else if (amount >= 1000) {
        return `£${(amount / 1000).toFixed(0)}K`;
    }
    return `£${amount}`;
}

function formatDate(date) {
    const months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
    return `${date.getDate()} ${months[date.getMonth()]} ${date.getFullYear()}`;
}

function randomInt(min, max) {
    return Math.floor(Math.random() * (max - min + 1)) + min;
}

function randomElement(arr) {
    return arr[Math.floor(Math.random() * arr.length)];
}

function generatePlayer(position, clubId) {
    const firstName = randomElement(GameData.playerNames.first);
    const lastName = randomElement(GameData.playerNames.last);
    const age = randomInt(18, 35);
    const rating = randomInt(60, 92);
    const wage = Math.round((rating * rating * 50) / 100) * 100;

    return {
        id: `${clubId}_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
        name: `${firstName} ${lastName}`,
        firstName,
        lastName,
        age,
        position,
        nationality: randomElement(GameData.nationalities),
        rating,
        form: randomInt(5, 10),
        morale: randomInt(60, 100),
        wage,
        value: wage * 52 * randomInt(3, 8)
    };
}

function generateSquad(clubId) {
    const squad = [];
    const positions = ["GK", "GK", "DEF", "DEF", "DEF", "DEF", "DEF", "MID", "MID", "MID", "MID", "MID", "MID", "ATT", "ATT", "ATT", "ATT"];

    positions.forEach(pos => {
        squad.push(generatePlayer(pos, clubId));
    });

    return squad;
}

function generateFixtures() {
    const fixtures = [];
    const clubs = GameData.clubs.map(c => c.id);
    let matchDate = new Date(GameState.currentDate);
    matchDate.setDate(matchDate.getDate() + 4); // First match in 4 days

    // Generate round-robin fixtures
    for (let round = 0; round < (clubs.length - 1) * 2; round++) {
        for (let i = 0; i < clubs.length / 2; i++) {
            const home = clubs[i];
            const away = clubs[clubs.length - 1 - i];

            if (round % 2 === 0) {
                fixtures.push({ home, away, date: new Date(matchDate), played: false, homeScore: null, awayScore: null });
            } else {
                fixtures.push({ home: away, away: home, date: new Date(matchDate), played: false, homeScore: null, awayScore: null });
            }
        }

        // Rotate clubs (keep first club fixed)
        clubs.splice(1, 0, clubs.pop());
        matchDate.setDate(matchDate.getDate() + 7);
    }

    return fixtures;
}

function initLeagueTable() {
    return GameData.clubs.map(club => ({
        clubId: club.id,
        name: club.name,
        played: 0,
        won: 0,
        drawn: 0,
        lost: 0,
        goalsFor: 0,
        goalsAgainst: 0,
        goalDifference: 0,
        points: 0
    }));
}

function updateLeagueTable(homeId, awayId, homeScore, awayScore) {
    const homeTeam = GameState.leagueTable.find(t => t.clubId === homeId);
    const awayTeam = GameState.leagueTable.find(t => t.clubId === awayId);

    homeTeam.played++;
    awayTeam.played++;
    homeTeam.goalsFor += homeScore;
    homeTeam.goalsAgainst += awayScore;
    awayTeam.goalsFor += awayScore;
    awayTeam.goalsAgainst += homeScore;

    if (homeScore > awayScore) {
        homeTeam.won++;
        homeTeam.points += 3;
        awayTeam.lost++;
    } else if (homeScore < awayScore) {
        awayTeam.won++;
        awayTeam.points += 3;
        homeTeam.lost++;
    } else {
        homeTeam.drawn++;
        awayTeam.drawn++;
        homeTeam.points++;
        awayTeam.points++;
    }

    homeTeam.goalDifference = homeTeam.goalsFor - homeTeam.goalsAgainst;
    awayTeam.goalDifference = awayTeam.goalsFor - awayTeam.goalsAgainst;

    // Sort table
    GameState.leagueTable.sort((a, b) => {
        if (b.points !== a.points) return b.points - a.points;
        if (b.goalDifference !== a.goalDifference) return b.goalDifference - a.goalDifference;
        return b.goalsFor - a.goalsFor;
    });
}

function showToast(message, type = "info") {
    const container = document.getElementById("toast-container");
    const toast = document.createElement("div");
    toast.className = `toast ${type}`;

    const icons = { success: "✓", error: "✕", info: "ℹ" };
    toast.innerHTML = `
        <span class="toast-icon">${icons[type]}</span>
        <span class="toast-message">${message}</span>
    `;

    container.appendChild(toast);

    setTimeout(() => {
        toast.style.animation = "slideIn 0.3s ease reverse";
        setTimeout(() => toast.remove(), 300);
    }, 3000);
}

function addNews(text) {
    const date = formatDate(GameState.currentDate);
    GameState.news.unshift({ date, text });
    renderNewsFeed();
}

// ============================================
// Rendering Functions
// ============================================

function renderTeamSelection() {
    const grid = document.getElementById("teams-grid");
    grid.innerHTML = "";

    GameData.clubs.forEach(club => {
        const stars = "★".repeat(Math.floor(club.reputation / 20));
        const card = document.createElement("div");
        card.className = "team-card";
        card.dataset.clubId = club.id;

        card.innerHTML = `
            <div class="team-badge" style="background: ${club.primary_color}">
                <span>${club.short_name.charAt(0)}</span>
            </div>
            <h3>${club.name}</h3>
            <div class="team-rep">
                <span class="stars">${stars}</span>
                <span>${club.reputation}</span>
            </div>
        `;

        card.addEventListener("click", () => selectTeam(club.id));
        grid.appendChild(card);
    });
}

function selectTeam(clubId) {
    document.querySelectorAll(".team-card").forEach(card => {
        card.classList.toggle("selected", card.dataset.clubId === clubId);
    });

    GameState.club = GameData.clubs.find(c => c.id === clubId);
    document.getElementById("start-game-btn").disabled = false;
}

function renderDashboard() {
    const club = GameState.club;
    const position = GameState.leagueTable.findIndex(t => t.clubId === club.id) + 1;
    const teamData = GameState.leagueTable.find(t => t.clubId === club.id);

    document.getElementById("club-position").textContent = `${position}${getOrdinal(position)}`;
    document.getElementById("club-points").textContent = teamData.points;

    // Form
    const formContainer = document.getElementById("club-form");
    if (GameState.form.length === 0) {
        formContainer.innerHTML = '<span class="form-badge">-</span>';
    } else {
        formContainer.innerHTML = GameState.form.slice(-5).map(f =>
            `<span class="form-badge ${f}">${f}</span>`
        ).join("");
    }

    // Next match
    const nextFixture = GameState.fixtures.find(f => !f.played && (f.home === club.id || f.away === club.id));
    if (nextFixture) {
        const opponent = nextFixture.home === club.id ?
            GameData.clubs.find(c => c.id === nextFixture.away).name :
            GameData.clubs.find(c => c.id === nextFixture.home).name;
        const venue = nextFixture.home === club.id ? "(H)" : "(A)";
        document.getElementById("next-match").textContent = `vs ${opponent} ${venue}`;
    } else {
        document.getElementById("next-match").textContent = "Season Complete";
    }

    // Finances
    document.getElementById("balance").textContent = formatCurrency(club.balance);
    document.getElementById("transfer-budget").textContent = formatCurrency(club.transfer_budget);
    document.getElementById("wage-budget").textContent = `${formatCurrency(club.wage_budget)}/week`;

    // Squad summary
    document.getElementById("squad-size").textContent = GameState.squad.length;
    const avgRating = Math.round(GameState.squad.reduce((sum, p) => sum + p.rating, 0) / GameState.squad.length);
    document.getElementById("squad-avg-rating").textContent = avgRating;
    document.getElementById("squad-injuries").textContent = "0";
}

function getOrdinal(n) {
    const s = ["th", "st", "nd", "rd"];
    const v = n % 100;
    return s[(v - 20) % 10] || s[v] || s[0];
}

function renderSquadTable() {
    const tbody = document.getElementById("squad-table-body");
    tbody.innerHTML = "";

    const filter = document.getElementById("squad-filter").value;
    let players = [...GameState.squad];

    if (filter !== "all") {
        players = players.filter(p => p.position === filter);
    }

    // Sort by position then rating
    const posOrder = { GK: 1, DEF: 2, MID: 3, ATT: 4 };
    players.sort((a, b) => {
        if (posOrder[a.position] !== posOrder[b.position]) {
            return posOrder[a.position] - posOrder[b.position];
        }
        return b.rating - a.rating;
    });

    players.forEach(player => {
        const moraleClass = player.morale >= 70 ? "high" : player.morale >= 40 ? "medium" : "low";
        const row = document.createElement("tr");
        row.innerHTML = `
            <td><span class="player-name">${player.name}</span></td>
            <td>${player.age}</td>
            <td><span class="position-badge ${player.position}">${player.position}</span></td>
            <td>${player.nationality}</td>
            <td><span class="rating-badge">${player.rating}</span></td>
            <td>${player.form}/10</td>
            <td>
                <div class="morale-bar">
                    <div class="morale-fill ${moraleClass}" style="width: ${player.morale}%"></div>
                </div>
            </td>
            <td>${formatCurrency(player.wage)}/w</td>
        `;
        tbody.appendChild(row);
    });
}

function renderFormation() {
    const container = document.getElementById("formation-players");
    const formation = GameData.formations[GameState.formation];

    container.innerHTML = "";

    formation.forEach((row, rowIndex) => {
        const rowDiv = document.createElement("div");
        rowDiv.className = "formation-row";

        row.players.forEach(player => {
            const playerDiv = document.createElement("div");
            playerDiv.className = `formation-player ${player.pos === "GK" ? "gk" : ""}`;
            playerDiv.textContent = player.num;
            playerDiv.title = player.pos;
            rowDiv.appendChild(playerDiv);
        });

        container.appendChild(rowDiv);
    });
}

function renderNextMatch() {
    const club = GameState.club;
    const nextFixture = GameState.fixtures.find(f => !f.played && (f.home === club.id || f.away === club.id));

    if (!nextFixture) {
        document.getElementById("next-match-card").innerHTML = `
            <div class="match-header">
                <span class="match-competition">Premier League</span>
            </div>
            <div class="match-teams">
                <p style="text-align: center; color: var(--text-secondary); padding: 40px;">
                    Season Complete - No more fixtures
                </p>
            </div>
        `;
        return;
    }

    const homeClub = GameData.clubs.find(c => c.id === nextFixture.home);
    const awayClub = GameData.clubs.find(c => c.id === nextFixture.away);

    document.getElementById("match-date").textContent = formatDate(nextFixture.date);
    document.getElementById("home-team").textContent = homeClub.name;
    document.getElementById("away-team").textContent = awayClub.name;
    document.getElementById("home-badge").style.background = homeClub.primary_color;
    document.getElementById("home-badge").querySelector("span").textContent = homeClub.short_name.charAt(0);
    document.getElementById("away-badge").style.background = awayClub.primary_color;
    document.getElementById("away-badge").querySelector("span").textContent = awayClub.short_name.charAt(0);
}

function renderLeagueTable() {
    const tbody = document.getElementById("league-table-body");
    tbody.innerHTML = "";

    GameState.leagueTable.forEach((team, index) => {
        const row = document.createElement("tr");
        if (team.clubId === GameState.club.id) {
            row.className = "your-team";
        }

        row.innerHTML = `
            <td>${index + 1}</td>
            <td>${team.name}</td>
            <td>${team.played}</td>
            <td>${team.won}</td>
            <td>${team.drawn}</td>
            <td>${team.lost}</td>
            <td>${team.goalsFor}</td>
            <td>${team.goalsAgainst}</td>
            <td>${team.goalDifference > 0 ? "+" : ""}${team.goalDifference}</td>
            <td><strong>${team.points}</strong></td>
        `;
        tbody.appendChild(row);
    });
}

function renderResults() {
    const container = document.getElementById("results-list");

    if (GameState.results.length === 0) {
        container.innerHTML = '<p class="no-results">No matches played yet</p>';
        return;
    }

    container.innerHTML = GameState.results.slice(-5).reverse().map(result => {
        const homeClub = GameData.clubs.find(c => c.id === result.home);
        const awayClub = GameData.clubs.find(c => c.id === result.away);

        let resultClass = "draw";
        if (result.home === GameState.club.id) {
            resultClass = result.homeScore > result.awayScore ? "win" : result.homeScore < result.awayScore ? "loss" : "draw";
        } else {
            resultClass = result.awayScore > result.homeScore ? "win" : result.awayScore < result.homeScore ? "loss" : "draw";
        }

        return `
            <div class="result-item">
                <span class="result-teams">${homeClub.short_name} vs ${awayClub.short_name}</span>
                <span class="result-final ${resultClass}">${result.homeScore} - ${result.awayScore}</span>
            </div>
        `;
    }).join("");
}

function renderTransferList() {
    const container = document.getElementById("transfer-list");

    if (GameState.transferTargets.length === 0) {
        // Generate some transfer targets from other clubs
        GameData.clubs.forEach(club => {
            if (club.id !== GameState.club.id) {
                for (let i = 0; i < 3; i++) {
                    const pos = randomElement(["DEF", "MID", "ATT"]);
                    const player = generatePlayer(pos, club.id);
                    player.club = club.name;
                    GameState.transferTargets.push(player);
                }
            }
        });
    }

    container.innerHTML = GameState.transferTargets.slice(0, 10).map(player => `
        <div class="transfer-player-card">
            <div class="transfer-player-info">
                <div class="transfer-player-avatar">${player.nationality}</div>
                <div class="transfer-player-details">
                    <h4>${player.name}</h4>
                    <span>${player.position} • ${player.age} • ${player.club}</span>
                </div>
            </div>
            <div class="transfer-player-value">${formatCurrency(player.value)}</div>
        </div>
    `).join("");
}

function renderNewsFeed() {
    const container = document.getElementById("news-feed");
    container.innerHTML = GameState.news.slice(0, 10).map(item => `
        <div class="news-item">
            <span class="news-date">${item.date}</span>
            <span class="news-text">${item.text}</span>
        </div>
    `).join("");
}

function renderFinances() {
    const club = GameState.club;
    const weeklyWages = GameState.squad.reduce((sum, p) => sum + p.wage, 0);

    document.getElementById("fin-balance").textContent = formatCurrency(club.balance);
    document.getElementById("fin-wages").textContent = `-${formatCurrency(weeklyWages)}`;
    document.getElementById("fin-transfer").textContent = formatCurrency(club.transfer_budget);
    document.getElementById("fin-wage-remaining").textContent = formatCurrency(club.wage_budget - weeklyWages);
}

// ============================================
// Match Simulation
// ============================================

function simulateMatch() {
    const club = GameState.club;
    const fixture = GameState.fixtures.find(f => !f.played && (f.home === club.id || f.away === club.id));

    if (!fixture) {
        showToast("No more fixtures to play!", "error");
        return;
    }

    const homeClub = GameData.clubs.find(c => c.id === fixture.home);
    const awayClub = GameData.clubs.find(c => c.id === fixture.away);

    // Show overlay
    const overlay = document.getElementById("match-overlay");
    overlay.classList.remove("hidden");

    // Setup match display
    document.getElementById("sim-home-name").textContent = homeClub.short_name;
    document.getElementById("sim-away-name").textContent = awayClub.short_name;
    document.getElementById("sim-home-badge").textContent = homeClub.short_name.charAt(0);
    document.getElementById("sim-home-badge").style.background = homeClub.primary_color;
    document.getElementById("sim-away-badge").textContent = awayClub.short_name.charAt(0);
    document.getElementById("sim-away-badge").style.background = awayClub.primary_color;
    document.getElementById("sim-home-score").textContent = "0";
    document.getElementById("sim-away-score").textContent = "0";
    document.getElementById("match-minute").textContent = "0'";
    document.getElementById("timer-progress").style.width = "0%";

    const commentary = document.getElementById("match-commentary");
    commentary.innerHTML = '<div class="commentary-item"><span class="comm-time">0\'</span><span class="comm-text">Match kicks off!</span></div>';

    const finishBtn = document.getElementById("finish-match-btn");
    finishBtn.disabled = true;
    finishBtn.textContent = "Match in Progress...";

    // Calculate match strength
    const homeStrength = homeClub.reputation + (fixture.home === club.id ? 5 : 0); // Home advantage
    const awayStrength = awayClub.reputation;

    let homeScore = 0;
    let awayScore = 0;
    let currentMinute = 0;
    const events = [];

    // Generate match events
    const numEvents = randomInt(5, 15);
    for (let i = 0; i < numEvents; i++) {
        const minute = randomInt(1, 90);
        const eventType = Math.random();

        if (eventType < 0.4) {
            // Goal
            const scorer = Math.random() < (homeStrength / (homeStrength + awayStrength)) ? "home" : "away";
            events.push({ minute, type: "goal", team: scorer });
        } else if (eventType < 0.6) {
            // Yellow card
            const team = Math.random() < 0.5 ? "home" : "away";
            events.push({ minute, type: "yellow", team });
        } else {
            // Chance
            const team = Math.random() < 0.5 ? "home" : "away";
            events.push({ minute, type: "chance", team });
        }
    }

    // Sort events by minute
    events.sort((a, b) => a.minute - b.minute);

    // Simulate match with commentary
    let eventIndex = 0;
    const matchInterval = setInterval(() => {
        currentMinute += randomInt(3, 8);
        if (currentMinute > 90) currentMinute = 90;

        document.getElementById("match-minute").textContent = `${currentMinute}'`;
        document.getElementById("timer-progress").style.width = `${(currentMinute / 90) * 100}%`;

        // Process events up to current minute
        while (eventIndex < events.length && events[eventIndex].minute <= currentMinute) {
            const event = events[eventIndex];
            const teamName = event.team === "home" ? homeClub.short_name : awayClub.short_name;

            let commentaryText = "";
            let commentClass = "";

            if (event.type === "goal") {
                if (event.team === "home") {
                    homeScore++;
                    document.getElementById("sim-home-score").textContent = homeScore;
                } else {
                    awayScore++;
                    document.getElementById("sim-away-score").textContent = awayScore;
                }
                commentaryText = `⚽ GOAL! ${teamName} scores!`;
                commentClass = "goal";
            } else if (event.type === "yellow") {
                commentaryText = `🟨 Yellow card for ${teamName}`;
                commentClass = "card";
            } else {
                commentaryText = `${teamName} creates a chance but it comes to nothing`;
            }

            const commentItem = document.createElement("div");
            commentItem.className = `commentary-item ${commentClass}`;
            commentItem.innerHTML = `<span class="comm-time">${event.minute}'</span><span class="comm-text">${commentaryText}</span>`;
            commentary.appendChild(commentItem);
            commentary.scrollTop = commentary.scrollHeight;

            eventIndex++;
        }

        if (currentMinute >= 90) {
            clearInterval(matchInterval);

            // Add final whistle
            const finalItem = document.createElement("div");
            finalItem.className = "commentary-item";
            finalItem.innerHTML = `<span class="comm-time">90'</span><span class="comm-text">Full time! ${homeClub.short_name} ${homeScore} - ${awayScore} ${awayClub.short_name}</span>`;
            commentary.appendChild(finalItem);

            finishBtn.disabled = false;
            finishBtn.textContent = "Continue";

            // Store result
            fixture.played = true;
            fixture.homeScore = homeScore;
            fixture.awayScore = awayScore;

            GameState.results.push({
                home: fixture.home,
                away: fixture.away,
                homeScore,
                awayScore,
                date: fixture.date
            });

            // Update form
            let formResult;
            if (fixture.home === club.id) {
                formResult = homeScore > awayScore ? "W" : homeScore < awayScore ? "L" : "D";
            } else {
                formResult = awayScore > homeScore ? "W" : awayScore < homeScore ? "L" : "D";
            }
            GameState.form.push(formResult);

            // Update league table
            updateLeagueTable(fixture.home, fixture.away, homeScore, awayScore);

            // Add news
            addNews(`${homeClub.name} ${homeScore} - ${awayScore} ${awayClub.name}. ${formResult === "W" ? "Great result!" : formResult === "L" ? "Disappointing result." : "A point gained."}`);
        }
    }, 500);
}

function finishMatch() {
    document.getElementById("match-overlay").classList.add("hidden");

    // Refresh UI
    renderDashboard();
    renderNextMatch();
    renderLeagueTable();
    renderResults();

    showToast("Match completed!", "success");
}

// ============================================
// Game Controls
// ============================================

function advanceDay() {
    GameState.currentDate.setDate(GameState.currentDate.getDate() + 1);
    document.getElementById("current-date").textContent = formatDate(GameState.currentDate);

    // Random events
    if (Math.random() < 0.2) {
        const events = [
            "Training session went well today. Players looking sharp.",
            "Scouts report new talent available in the transfer market.",
            "Team morale is good heading into the next match.",
            "The board is pleased with recent performances.",
            "Fans are excited about the upcoming fixtures."
        ];
        addNews(randomElement(events));
    }

    // Check for match day
    const todayFixture = GameState.fixtures.find(f =>
        !f.played &&
        formatDate(f.date) === formatDate(GameState.currentDate) &&
        (f.home === GameState.club.id || f.away === GameState.club.id)
    );

    if (todayFixture) {
        showToast("Match day! Time to play.", "info");
        switchPanel("matches");
    }

    renderDashboard();
}

function switchPanel(panelName) {
    // Update nav
    document.querySelectorAll(".nav-item").forEach(item => {
        item.classList.toggle("active", item.dataset.screen === panelName);
    });

    // Show panel
    document.querySelectorAll(".panel").forEach(panel => {
        panel.classList.toggle("active", panel.id === `${panelName}-panel`);
    });

    // Render panel content
    switch (panelName) {
        case "dashboard":
            renderDashboard();
            break;
        case "squad":
            renderSquadTable();
            break;
        case "tactics":
            renderFormation();
            break;
        case "matches":
            renderNextMatch();
            renderResults();
            break;
        case "table":
            renderLeagueTable();
            break;
        case "transfers":
            renderTransferList();
            break;
        case "finances":
            renderFinances();
            break;
    }
}

// ============================================
// Game Initialization
// ============================================

function startGame() {
    const managerName = document.getElementById("manager-name").value.trim() || "Manager";
    GameState.manager = managerName;

    // Generate squad
    GameState.squad = generateSquad(GameState.club.id);

    // Generate fixtures
    GameState.fixtures = generateFixtures();

    // Initialize league table
    GameState.leagueTable = initLeagueTable();

    // Initialize news
    GameState.news = [
        { date: formatDate(GameState.currentDate), text: `Welcome to ${GameState.club.name}! Your managerial career begins today.` },
        { date: formatDate(GameState.currentDate), text: `The board expects a top 4 finish this season.` }
    ];

    // Update sidebar
    document.getElementById("sidebar-club-name").textContent = GameState.club.name;
    document.getElementById("sidebar-manager-name").textContent = managerName;
    document.getElementById("sidebar-badge").style.background = GameState.club.primary_color;
    document.getElementById("sidebar-badge").querySelector(".badge-letter").textContent = GameState.club.short_name.charAt(0);
    document.getElementById("current-date").textContent = formatDate(GameState.currentDate);

    // Hide team select, show game
    document.getElementById("team-select-screen").classList.add("hidden");
    document.getElementById("game-screen").classList.remove("hidden");

    // Render initial UI
    renderDashboard();
    renderNewsFeed();
    renderFormation();

    showToast(`Welcome to ${GameState.club.name}!`, "success");
}

function initGame() {
    // Render team selection
    renderTeamSelection();

    // Setup event listeners
    document.getElementById("start-game-btn").addEventListener("click", startGame);

    // Navigation
    document.querySelectorAll(".nav-item").forEach(item => {
        item.addEventListener("click", () => switchPanel(item.dataset.screen));
    });

    // Advance day
    document.getElementById("advance-day-btn").addEventListener("click", advanceDay);

    // Match simulation
    document.getElementById("simulate-match-btn").addEventListener("click", simulateMatch);
    document.getElementById("finish-match-btn").addEventListener("click", finishMatch);

    // Formation buttons
    document.querySelectorAll(".formation-btn").forEach(btn => {
        btn.addEventListener("click", () => {
            document.querySelectorAll(".formation-btn").forEach(b => b.classList.remove("active"));
            btn.classList.add("active");
            GameState.formation = btn.dataset.formation;
            renderFormation();
        });
    });

    // Tempo buttons
    document.querySelectorAll(".tempo-btn").forEach(btn => {
        btn.addEventListener("click", () => {
            document.querySelectorAll(".tempo-btn").forEach(b => b.classList.remove("active"));
            btn.classList.add("active");
            GameState.tempo = btn.dataset.tempo;
        });
    });

    // Sliders
    document.getElementById("pressing-slider").addEventListener("input", (e) => {
        GameState.pressing = e.target.value;
        document.getElementById("pressing-value").textContent = `${e.target.value}%`;
    });

    document.getElementById("mentality-slider").addEventListener("input", (e) => {
        GameState.mentality = e.target.value;
    });

    // Squad filter
    document.getElementById("squad-filter").addEventListener("change", renderSquadTable);

    // Save tactics button
    document.querySelector(".save-tactics-btn").addEventListener("click", () => {
        showToast("Tactics saved!", "success");
    });

    // Hide loading screen after setup
    setTimeout(() => {
        const loading = document.getElementById("loading-screen");
        loading.classList.add("fade-out");
        document.getElementById("team-select-screen").classList.remove("hidden");
    }, 2000);
}

// Start the game when DOM is ready
document.addEventListener("DOMContentLoaded", initGame);
