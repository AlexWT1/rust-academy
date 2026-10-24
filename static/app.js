/* Rust Academy — подсветка синтаксиса, квизы, прогресс с синхронизацией */
(function () {
  "use strict";

  /* ---------- Подсветка Rust ---------- */
  const KEYWORDS = new Set([
    "as", "async", "await", "break", "const", "continue", "crate", "dyn",
    "else", "enum", "extern", "false", "fn", "for", "if", "impl", "in",
    "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type",
    "unsafe", "use", "where", "while",
  ]);

  function esc(s) {
    return s.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");
  }

  function span(cls, text) {
    return '<span class="tk-' + cls + '">' + esc(text) + "</span>";
  }

  function highlight(src) {
    let out = "";
    let i = 0;
    const n = src.length;
    while (i < n) {
      const rest = src.slice(i);
      let m;

      if (src.startsWith("//", i)) {
        let j = src.indexOf("\n", i);
        if (j === -1) j = n;
        out += span("cmt", src.slice(i, j));
        i = j;
        continue;
      }
      if (src.startsWith("/*", i)) {
        let j = src.indexOf("*/", i + 2);
        j = j === -1 ? n : j + 2;
        out += span("cmt", src.slice(i, j));
        i = j;
        continue;
      }
      if (src[i] === "#") {
        m = /^#!?\[[^\]]*\]/.exec(rest);
        if (m) {
          out += span("attr", m[0]);
          i += m[0].length;
          continue;
        }
      }
      if (src[i] === '"') {
        let j = i + 1;
        while (j < n && src[j] !== '"') {
          if (src[j] === "\\") j++;
          j++;
        }
        out += span("str", src.slice(i, Math.min(j + 1, n)));
        i = j + 1;
        continue;
      }
      if (src[i] === "'") {
        m = /^'(\\.|[^'\\])'/.exec(rest); // символ
        if (m) {
          out += span("str", m[0]);
          i += m[0].length;
          continue;
        }
        m = /^'[a-zA-Z_][a-zA-Z0-9_]*\b/.exec(rest); // lifetime
        if (m) {
          out += span("life", m[0]);
          i += m[0].length;
          continue;
        }
      }
      m = /^\d[\d_]*(?:\.\d+)?(?:[eE][+-]?\d+)?(?:[a-zA-Z_]\w*)?/.exec(rest); // число
      if (m) {
        out += span("num", m[0]);
        i += m[0].length;
        continue;
      }
      m = /^[A-Za-z_]\w*/.exec(rest); // идентификатор
      if (m) {
        const word = m[0];
        let after = i + word.length;
        if (src[after] === "!") {
          out += span("macro", word + "!");
          i = after + 1;
        } else if (KEYWORDS.has(word)) {
          out += span("kw", word);
          i = after;
        } else if (/^[A-Z]/.test(word)) {
          out += span("type", word);
          i = after;
        } else if (src[after] === "(" || src.slice(after, after + 2) === "::") {
          out += span("fn", word);
          i = after;
        } else {
          out += esc(word);
          i = after;
        }
        continue;
      }
      // прочее — операторы и пунктуация
      m = /^[^\w"'\n]+/.exec(rest);
      if (m) {
        out += span("op", m[0]);
        i += m[0].length;
      } else {
        out += esc(src[i]);
        i++;
      }
    }
    return out;
  }

  function highlightAll(root) {
    root.querySelectorAll("pre code.rust").forEach(function (el) {
      el.innerHTML = highlight(el.textContent);
    });
  }

  /* ---------- Прогресс: сервер + localStorage-кэш ---------- */
  const AUTH = document.body.dataset.auth === "1";
  const DONE_PREFIX = "ra_done_";
  let DONE = new Set(); // актуальное множество пройденных уроков

  function readLocalDone() {
    const set = new Set();
    for (let i = 0; i < localStorage.length; i++) {
      const k = localStorage.key(i);
      if (k && k.startsWith(DONE_PREFIX)) set.add(k.slice(DONE_PREFIX.length));
    }
    return set;
  }

  function writeLocalDone(set) {
    for (let i = localStorage.length - 1; i >= 0; i--) {
      const k = localStorage.key(i);
      if (k && k.startsWith(DONE_PREFIX)) localStorage.removeItem(k);
    }
    set.forEach(function (id) {
      localStorage.setItem(DONE_PREFIX + id, "1");
    });
  }

  function pushToServer(lessonId, done) {
    if (!AUTH) return;
    fetch("/api/progress", {
      method: "POST",
      credentials: "same-origin",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ lesson_id: lessonId, done: done }),
    }).catch(function () {
      /* офлайн — прогресс останется в localStorage и подтянется позже */
    });
  }

  function pushBatchToServer(items) {
    if (!AUTH || items.length === 0) return;
    fetch("/api/progress", {
      method: "POST",
      credentials: "same-origin",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ items: items }),
    }).catch(function () {});
  }

  async function loadDone() {
    const local = readLocalDone();
    if (!AUTH) return local;
    try {
      const resp = await fetch("/api/progress", { credentials: "same-origin" });
      if (!resp.ok) return local;
      const server = new Set(await resp.json());
      // локальные отметки, которых нет на сервере (сделаны офлайн) — залить
      const missing = [];
      local.forEach(function (id) {
        if (!server.has(id)) missing.push({ lesson_id: id, done: true });
      });
      pushBatchToServer(missing);
      const union = new Set([...local, ...server]);
      writeLocalDone(union);
      return union;
    } catch (e) {
      return local; // сервер недоступен — работаем по кэшу
    }
  }

  function refreshProgress() {
    const ids = new Set();
    document.querySelectorAll("[data-lesson]").forEach(function (el) {
      ids.add(el.dataset.lesson);
    });

    let doneCount = 0;
    ids.forEach(function (id) {
      if (DONE.has(id)) doneCount++;
    });

    const overallBar = document.getElementById("overall-bar");
    const overallLabel = document.getElementById("overall-label");
    const total = ids.size;
    if (overallBar && total > 0) {
      overallBar.style.width = (doneCount / total) * 100 + "%";
    }
    if (overallLabel) {
      overallLabel.textContent = "прогресс: " + doneCount + "/" + total;
    }
    const heroDone = document.getElementById("hero-done");
    if (heroDone) heroDone.textContent = String(doneCount);

    // точки у ссылок
    document.querySelectorAll("[data-lesson]").forEach(function (el) {
      const dot = el.querySelector(".dot");
      if (dot) dot.classList.toggle("done-dot", DONE.has(el.dataset.lesson));
    });

    // счётчики модулей на главной
    document.querySelectorAll(".module-card[data-module]").forEach(function (card) {
      const links = card.querySelectorAll(".lesson-link[data-lesson]");
      const counter = card.querySelector(".module-progress");
      if (!counter) return;
      let c = 0;
      links.forEach(function (l) {
        if (DONE.has(l.dataset.lesson)) c++;
      });
      counter.textContent = c + "/" + links.length;
      counter.style.color = c === links.length && links.length > 0 ? "var(--green)" : "";
    });
  }

  function toggleDone(id) {
    const nowDone = !DONE.has(id);
    if (nowDone) DONE.add(id);
    else DONE.delete(id);
    writeLocalDone(DONE);
    pushToServer(id, nowDone);
    refreshProgress();
  }

  /* ---------- Кнопка «пройдено» ---------- */
  function initDoneButton() {
    const btn = document.getElementById("done-btn");
    if (!btn) return;
    const id = btn.dataset.lesson;
    const sync = function () {
      const isDone = DONE.has(id);
      btn.classList.toggle("is-done", isDone);
      btn.textContent = isDone ? "✔ Урок пройден" : "✔ Отметить пройденным";
    };
    btn.addEventListener("click", function () {
      toggleDone(id);
      sync();
    });
    sync();
  }

  /* ---------- Квизы ---------- */
  function initQuiz() {
    document.querySelectorAll(".quiz-q").forEach(function (form) {
      form.addEventListener("submit", function (e) {
        e.preventDefault();
        const answer = parseInt(form.dataset.answer, 10);
        const checked = form.querySelector("input:checked");
        const result = form.querySelector(".quiz-result");
        const explain = form.querySelector(".quiz-explain");
        if (!checked) {
          result.hidden = false;
          result.className = "quiz-result bad";
          result.textContent = "Выберите вариант 👆";
          return;
        }
        const picked = parseInt(checked.value, 10);
        const isRight = picked === answer;
        form.querySelectorAll(".quiz-opt").forEach(function (opt) {
          opt.classList.remove("correct", "wrong");
          const input = opt.querySelector("input");
          if (parseInt(input.value, 10) === answer) opt.classList.add("correct");
          else if (input.checked) opt.classList.add("wrong");
          input.disabled = true;
        });
        form.querySelector(".check-btn").disabled = true;
        result.hidden = false;
        result.className = "quiz-result " + (isRight ? "ok" : "bad");
        result.textContent = isRight ? "✅ Верно!" : "❌ Не совсем — правильный ответ подсвечен.";
        explain.hidden = false;
      });
    });
  }

  /* ---------- Playground ---------- */
  function openInPlayground(code) {
    const url =
      "https://play.rust-lang.org/?edition=2021&mode=debug&code=" +
      encodeURIComponent(code);
    window.open(url, "_blank", "noopener");
  }

  function initPlayground() {
    document.querySelectorAll(".play-btn").forEach(function (btn) {
      const box = btn.closest(".codebox");
      if (!box) return;
      const code = box.querySelector("code");
      if (!code) return;
      btn.style.cursor = "pointer";
      btn.addEventListener("click", function () {
        openInPlayground(code.textContent);
      });
    });
  }

  /* ---------- Старт ---------- */
  highlightAll(document);
  initPlayground();
  initQuiz();
  loadDone().then(function (done) {
    DONE = done;
    initDoneButton();
    refreshProgress();
  });
})();
