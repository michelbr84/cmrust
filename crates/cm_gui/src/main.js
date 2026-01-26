const { invoke } = window.__TAURI__.core;

const app = {
  state: {
    language: 'en',
    strings: {},
    newGameData: {
      teamPool: [], // All 30 teams
      randomTeams: [] // Selected 6 teams
    },
    gameState: null
  },

  init: async () => {
    await app.loadFlags();
  },

  // --- Loader Helper ---
  loadJSON: async (path) => {
    try {
      const res = await fetch(path);
      if (!res.ok) throw new Error(`Failed to load ${path}`);
      return await res.json();
    } catch (e) {
      console.error(e);
      return null;
    }
  },

  // --- Language ---
  loadFlags: async () => {
    const data = await app.loadJSON('assets/JSON/flags.json');
    if (data) app.renderFlags(data.paises);
  },

  renderFlags: (countries) => {
    const grid = document.getElementById('flags-grid');
    grid.innerHTML = '';
    countries.forEach(country => {
      const btn = document.createElement('div');
      btn.className = 'flag-btn';
      btn.innerHTML = `
        <div class="flag-emoji">${country.flag}</div>
        <div class="flag-name">${country.nome}</div>
      `;
      btn.onclick = () => app.selectLanguage(country.language.code);
      grid.appendChild(btn);
    });
  },

  selectLanguage: async (code) => {
    app.state.language = code;
    await app.loadStrings(code);
    app.showScreen('start');
  },

  loadStrings: async (code) => {
    // Try specific -> base -> en
    const paths = [
      `assets/JSON/${code}/start.json`,
      `assets/JSON/${code.split('-')[0]}/start.json`,
      `assets/JSON/en/start.json`
    ];

    let data = null;
    for (const p of paths) {
      data = await app.loadJSON(p);
      if (data) break;
    }

    if (data) {
      app.state.strings = data;
      app.renderStartMenu(data);
    }
  },

  // --- Start Menu ---
  renderStartMenu: (data) => {
    // Update title if exists in data, else keep static
    const container = document.getElementById('menu-buttons');
    container.innerHTML = '';
    if (data.menu_inicial) {
      data.menu_inicial.forEach(item => {
        const btn = document.createElement('button');
        btn.className = 'menu-btn';
        btn.textContent = item.label;
        btn.onclick = () => app.handleMenuAction(item.id);
        container.appendChild(btn);
      });
    }
  },

  handleMenuAction: (actionId) => {
    switch (actionId) {
      case 'start_game':
        app.prepNewGame();
        break;
      case 'continue_game':
        alert("Load Game Not Implemented yet.");
        break;
      case 'options':
        app.showScreen('options');
        break;
      case 'exit':
        // Exit logic (window.close())
        break;
    }
  },

  // --- New Game ---
  prepNewGame: async () => {
    // 1. Load Times for current language
    const lang = app.state.language;
    // Fallback logic for times.json path
    const paths = [
      `assets/JSON/${lang}/times.json`,
      `assets/JSON/${lang.split('-')[0]}/times.json`,
      `assets/JSON/pt-BR/times.json` // Default to PT-BR as verified resource
    ];

    let teamsData = null;
    for (const p of paths) {
      teamsData = await app.loadJSON(p);
      if (teamsData) break;
    }

    if (!teamsData) {
      alert("Error loading teams data.");
      return;
    }

    app.state.newGameData.teamPool = teamsData.times;

    // 2. Select 6 Random Teams (Fisher-Yates Shuffle or Set)
    const pool = [...app.state.newGameData.teamPool];
    for (let i = pool.length - 1; i > 0; i--) {
      const j = Math.floor(Math.random() * (i + 1));
      [pool[i], pool[j]] = [pool[j], pool[i]];
    }
    app.state.newGameData.randomTeams = pool.slice(0, 6);

    // 3. Populate Form (Nations)
    const natSelect = document.getElementById('manager-nationality');
    if (natSelect.children.length === 0) {
      // Reuse flags.json or paises.json
      const flagsData = await app.loadJSON('assets/JSON/flags.json');
      if (flagsData) {
        flagsData.paises.sort((a, b) => a.nome.localeCompare(b.nome));
        flagsData.paises.forEach(p => {
          const opt = document.createElement('option');
          opt.value = p.nome; // Using Name as ID for now (simplification)
          opt.textContent = `${p.flag} ${p.nome}`;
          natSelect.appendChild(opt);
        });
      }
    }

    // 4. Render 3x2 Grid
    app.renderTeamGrid();
    app.showScreen('newgame');
  },

  renderTeamGrid: () => {
    const grid = document.getElementById('team-selection-grid');
    grid.innerHTML = '';
    const teams = app.state.newGameData.randomTeams;

    teams.forEach(team => {
      const card = document.createElement('div');
      card.className = 'team-card';
      card.dataset.id = team.id;
      card.onclick = () => app.selectTeam(team.id);

      card.innerHTML = `
            <div style="font-weight:700; font-size:1.1rem;">${team.nome}</div>
            <div class="team-color-strip" style="background: linear-gradient(to right, ${team.corPrimaria}, ${team.corSecundaria})"></div>
        `;
      grid.appendChild(card);
    });

    // Reset selection input
    document.getElementById('selected-team-id').value = '';
    document.getElementById('btn-start-career').disabled = true;
  },

  selectTeam: (teamId) => {
    // UI Update
    document.querySelectorAll('.team-card').forEach(el => {
      el.classList.toggle('selected', el.dataset.id == teamId);
    });

    // State Update
    document.getElementById('selected-team-id').value = teamId;
    document.getElementById('btn-start-career').disabled = false;
  },

  // --- World Generation & Simulation ---
  createCareer: async () => {
    const name = document.getElementById('manager-name').value;
    const surname = document.getElementById('manager-surname').value;
    const teamId = document.getElementById('selected-team-id').value;

    if (!name || !surname || !teamId) {
      alert("Please fill all fields and select a team.");
      return;
    }

    const selectedTeam = app.state.newGameData.randomTeams.find(t => t.id == teamId);

    // Load Template Data
    const templatePath = `assets/JSON/pt-BR/atual.json`; // Defaulting to PT-BR template
    const template = await app.loadJSON(templatePath);

    if (!template) {
      alert("Failed to load game template.");
      return;
    }

    // Construct Partial Game State (Mocking backend logic)
    app.state.gameState = {
      meta: {
        managerName: `${name} ${surname}`,
        clubId: teamId,
        clubName: selectedTeam.nome,
        startDate: new Date().toISOString()
      },
      // We clone the template structure
      ...template
    };

    // Update Template with Context
    app.state.gameState.game.dayLabel = new Date().toDateString(); // Simplified date

    // Inject custom welcome message
    app.state.gameState.messages = [
      {
        id: `msg-${Date.now()}`,
        type: "system",
        title: `Welcome to ${selectedTeam.nome}`,
        text: `Chairman welcomes ${name} ${surname} to the club. The fans are expecting great results this season!`,
        date: "2026-01-01",
        time: "09:00",
        unread: true
      }
    ];

    // Transition
    app.renderNewsScreen();
    app.showScreen('news');
  },

  // --- News Screen ---
  renderNewsScreen: () => {
    const state = app.state.gameState;

    // Header
    document.getElementById('news-club-name').textContent = state.meta.clubName;
    document.getElementById('game-date').textContent = state.game.dayLabel;

    // Inbox List
    const list = document.getElementById('news-inbox-list');
    list.innerHTML = '';

    state.messages.forEach(msg => {
      const item = document.createElement('div');
      item.className = `msg-item ${msg.unread ? 'unread' : ''}`;
      item.innerHTML = `
            <div class="msg-title">${msg.type === 'system' ? '📢 ' : '✉️ '}${msg.title}</div>
            <div class="msg-meta">${msg.time}</div>
        `;
      item.onclick = () => app.openMessage(msg);
      list.appendChild(item);
    });

    // Select first message automatically
    if (state.messages.length > 0) {
      app.openMessage(state.messages[0]);
    }
  },

  openMessage: (msg) => {
    const view = document.getElementById('news-article-view');
    view.innerHTML = `
        <div class="article-title">${msg.title}</div>
        <div class="article-meta" style="color:#888; margin-bottom:1rem; font-size:0.9rem;">
            ${msg.date} - ${msg.time} | Tags: ${(msg.tags || []).join(', ')}
        </div>
        <div class="article-body">
            ${msg.text}
        </div>
    `;

    // Mark as read visually (logic would update state)
    // Find item in list and remove unread class
  },

  changeLanguage: async (code) => {
    await app.selectLanguage(code);
    app.showScreen('options'); // Return to options after reload strings logic if needed
  },

  showScreen: (screenId) => {
    document.querySelectorAll('.screen').forEach(el => el.classList.remove('active'));
    document.getElementById(`screen-${screenId}`).classList.add('active');
  }
};

window.app = app;
window.addEventListener('DOMContentLoaded', app.init);
