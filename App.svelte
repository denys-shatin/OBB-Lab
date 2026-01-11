<script>
  import { onMount } from 'svelte';
  import * as THREE from 'three';
  import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js';
  import { TransformControls } from 'three/examples/jsm/controls/TransformControls.js';

  let canvasContainer;
  let scene, camera, renderer, controls, transformControl, raycaster, mouse;
  let modelMeshes = []; 
  let selectedMeshes = new Set();
  let invisiblePlane;

  // Ручной бокс
  let manualBox = null;
  let ghostBox = null;
  let creationPhase = 0;
  let startPoint = new THREE.Vector3();

  // Сидения
  let seats = [];
  let seatMode = false;
  let selectedSeatIndex = -1;

  // WASD движение
  let keys = { w: false, a: false, s: false, d: false, shift: false, space: false, ctrl: false };

  let modelData = null;
  let bonesList = [];
  let outputText = '';
  let errorMessage = '';
  
  // Режимы
  let selectionMode = 'cube';
  let editMode = 'scale';
  let cameraMode = false; // true = WASD mode, false = selection
  let isDragging = false;
  let moveSpeed = 0.15; // базовая скорость WASD

  // История для Ctrl+Z
  let history = [];
  const MAX_HISTORY = 50;

  // Локализация
  let currentLang = 'ru';
  let showTutorial = false;
  
  const translations = {
    ru: {
      title: 'OBB Lab',
      upload: 'Загрузить',
      undo: 'Ctrl+Z отмена',
      camera: 'Камера',
      select: 'Выбор',
      wasd: 'WASD',
      speed: 'Скорость',
      selection: 'Выделение',
      cube: 'Куб',
      bone: 'Кость',
      tools: 'Инструменты',
      all: 'Всё',
      reset: 'Сброс',
      drawBox: 'Нарисовать бокс',
      drawing: 'Рисование...',
      addSeat: 'Добавить сидение',
      disableSeat: 'Выкл. сидения',
      manualBox: 'Ручной бокс',
      delete: 'Удалить',
      seats: 'Сидения',
      result: 'Результат',
      copy: 'Копировать',
      loadFile: 'Загрузи .geo.json файл',
      clickSelect: 'Клик = выделить',
      selected: 'Выделено',
      wasdMode: 'WASD режим · Мышь вращение · Ctrl быстрее',
      clickForSeat: 'Кликай по модели для сидения',
      clickToDraw: 'Кликай для рисования · Esc отмена',
      manualBoxHint: 'Ручной бокс · T Move · R Scale · Del удалить',
      tutorial: 'Туториал',
      tutorialTitle: 'Как пользоваться',
      tutorialItems: [
        { title: 'Загрузка модели', desc: 'Нажми "Загрузить" и выбери .geo.json файл модели Minecraft Bedrock' },
        { title: 'Выделение', desc: 'Кликай по кубам модели чтобы выделить их. Режим "Кость" выделяет всю кость целиком' },
        { title: 'Ручной бокс', desc: 'Кнопка "Нарисовать бокс" позволяет создать свой хитбокс. Кликни 2 раза для создания' },
        { title: 'Сидения', desc: 'Режим сидений добавляет точки посадки игроков. Кликни по модели для размещения' },
        { title: 'WASD камера', desc: 'Режим WASD позволяет летать по сцене как в играх. Ctrl = быстрее. ⚠️ ВАЖНО: После использования WASD лучше оставайтесь в этом режиме, так как камера может забагаться в режиме выбора. Для исправления обновите страницу' },
        { title: 'Отмена', desc: 'Ctrl+Z отменяет последнее действие' },
        { title: 'Результат', desc: 'JSON код автоматически генерируется. Нажми "Копировать" для копирования' }
      ],
      close: 'Закрыть'
    },
    en: {
      title: 'OBB Lab',
      upload: 'Upload',
      undo: 'Ctrl+Z undo',
      camera: 'Camera',
      select: 'Select',
      wasd: 'WASD',
      speed: 'Speed',
      selection: 'Selection',
      cube: 'Cube',
      bone: 'Bone',
      tools: 'Tools',
      all: 'All',
      reset: 'Reset',
      drawBox: 'Draw box',
      drawing: 'Drawing...',
      addSeat: 'Add seat',
      disableSeat: 'Disable seats',
      manualBox: 'Manual box',
      delete: 'Delete',
      seats: 'Seats',
      result: 'Result',
      copy: 'Copy',
      loadFile: 'Load .geo.json file',
      clickSelect: 'Click = select',
      selected: 'Selected',
      wasdMode: 'WASD mode · Mouse rotate · Ctrl faster',
      clickForSeat: 'Click on model to add seat',
      clickToDraw: 'Click to draw · Esc cancel',
      manualBoxHint: 'Manual box · T Move · R Scale · Del delete',
      tutorial: 'Tutorial',
      tutorialTitle: 'How to use',
      tutorialItems: [
        { title: 'Load model', desc: 'Click "Upload" and select a Minecraft Bedrock .geo.json model file' },
        { title: 'Selection', desc: 'Click on model cubes to select them. "Bone" mode selects the entire bone' },
        { title: 'Manual box', desc: '"Draw box" button lets you create custom hitbox. Click twice to create' },
        { title: 'Seats', desc: 'Seat mode adds player seating points. Click on model to place' },
        { title: 'WASD camera', desc: 'WASD mode lets you fly around the scene like in games. Ctrl = faster. ⚠️ WARNING: After using WASD, it\'s better to stay in this mode as the camera may bug in selection mode. Refresh the page to fix' },
        { title: 'Undo', desc: 'Ctrl+Z undoes the last action' },
        { title: 'Result', desc: 'JSON code is generated automatically. Click "Copy" to copy' }
      ],
      close: 'Close'
    },
    ja: {
      title: 'OBB Lab',
      upload: 'アップロード',
      undo: 'Ctrl+Z 元に戻す',
      camera: 'カメラ',
      select: '選択',
      wasd: 'WASD',
      speed: '速度',
      selection: '選択モード',
      cube: 'キューブ',
      bone: 'ボーン',
      tools: 'ツール',
      all: 'すべて',
      reset: 'リセット',
      drawBox: 'ボックスを描く',
      drawing: '描画中...',
      addSeat: 'シートを追加',
      disableSeat: 'シート無効',
      manualBox: '手動ボックス',
      delete: '削除',
      seats: 'シート',
      result: '結果',
      copy: 'コピー',
      loadFile: '.geo.jsonファイルを読み込む',
      clickSelect: 'クリック = 選択',
      selected: '選択済み',
      wasdMode: 'WASDモード · マウス回転 · Ctrl 高速',
      clickForSeat: 'モデルをクリックしてシートを追加',
      clickToDraw: 'クリックして描画 · Esc キャンセル',
      manualBoxHint: '手動ボックス · T 移動 · R スケール · Del 削除',
      tutorial: 'チュートリアル',
      tutorialTitle: '使い方',
      tutorialItems: [
        { title: 'モデルの読み込み', desc: '「アップロード」をクリックしてMinecraft Bedrockの.geo.jsonモデルファイルを選択' },
        { title: '選択', desc: 'モデルのキューブをクリックして選択。「ボーン」モードはボーン全体を選択' },
        { title: '手動ボックス', desc: '「ボックスを描く」ボタンでカスタムヒットボックスを作成。2回クリックで作成' },
        { title: 'シート', desc: 'シートモードはプレイヤーの座席ポイントを追加。モデルをクリックして配置' },
        { title: 'WASDカメラ', desc: 'WASDモードでゲームのようにシーンを飛び回れます。Ctrl = 高速。⚠️ 警告：WASD使用後は選択モードでカメラがバグる可能性があるため、このモードに留まることをお勧めします。修正するにはページを更新してください' },
        { title: '元に戻す', desc: 'Ctrl+Zで最後のアクションを元に戻す' },
        { title: '結果', desc: 'JSONコードは自動生成されます。「コピー」をクリックしてコピー' }
      ],
      close: '閉じる'
    }
  };

  $: t = translations[currentLang];

  function detectLanguage() {
    const lang = navigator.language || navigator.userLanguage;
    const saved = localStorage.getItem('obb-lang');
    if (saved && translations[saved]) {
      currentLang = saved;
      return;
    }
    
    if (lang.startsWith('ja')) {
      currentLang = 'ja';
    } else if (lang.startsWith('ru') || lang.startsWith('uk') || lang.startsWith('be') || lang.startsWith('kk')) {
      currentLang = 'ru';
    } else {
      currentLang = 'en';
    }
  }

  function setLang(lang) {
    currentLang = lang;
    localStorage.setItem('obb-lang', lang);
  }

  const DEG2RAD = Math.PI / 180;
  const COLOR_SELECTED = 0x00ff00;
  const COLOR_SEAT = 0x00aaff;

  // Цвета для костей (как в Blockbench)
  const BONE_COLORS = [
    0x4285f4, // синий
    0xea4335, // красный
    0xfbbc05, // желтый
    0x34a853, // зеленый
    0xff6d01, // оранжевый
    0x46bdc6, // бирюзовый
    0xab47bc, // фиолетовый
    0x7baaf7, // голубой
    0xf07b72, // розовый
    0xfdd663, // светло-желтый
    0x57bb8a, // мятный
    0xffab40, // светло-оранжевый
    0x9575cd, // лавандовый
    0x4dd0e1, // циан
    0xf48fb1, // розовый
    0xaed581, // лайм
  ];
  let boneColorMap = {};

  onMount(() => {
    detectLanguage();
    initThreeJS();
    window.addEventListener('resize', onWindowResize);
    window.addEventListener('keydown', onKeyDown);
    window.addEventListener('keyup', onKeyUp);
    return () => {
      window.removeEventListener('resize', onWindowResize);
      window.removeEventListener('keydown', onKeyDown);
      window.removeEventListener('keyup', onKeyUp);
      if (renderer) renderer.dispose();
    };
  });

  function saveHistory() {
    const state = {
      selectedMeshes: new Set(selectedMeshes),
      hasManualBox: !!manualBox,
      manualBoxPos: manualBox ? manualBox.position.clone() : null,
      manualBoxScale: manualBox ? manualBox.scale.clone() : null,
      seats: seats.map(s => ({ 
        position: s.mesh.position.clone(), 
        boneName: s.boneName 
      }))
    };
    history.push(state);
    if (history.length > MAX_HISTORY) history.shift();
    console.log('History saved, length:', history.length);
  }

  function undo() {
    console.log('Undo called, history length:', history.length);
    if (history.length === 0) return;
    const state = history.pop();
    
    // Сбрасываем текущее выделение
    modelMeshes.forEach(mesh => {
      const boneColor = boneColorMap[mesh.userData?.boneName] || 0x888888;
      mesh.material.color.setHex(boneColor);
      mesh.material.emissive.setHex(0x000000);
    });
    
    // Восстанавливаем выделение из истории
    selectedMeshes = new Set(state.selectedMeshes);
    selectedMeshes.forEach(meshId => {
      const mesh = modelMeshes.find(m => m.uuid === meshId);
      if (mesh) {
        mesh.material.color.setHex(COLOR_SELECTED);
        mesh.material.emissive.setHex(0x003300);
      }
    });

    // Восстанавливаем ручной бокс
    if (state.hasManualBox) {
      if (!manualBox) {
        const geo = new THREE.BoxGeometry(1, 1, 1);
        const mat = new THREE.MeshStandardMaterial({ 
          color: 0xff4444, 
          transparent: true, 
          opacity: 0.6,
          emissive: 0x331111
        });
        manualBox = new THREE.Mesh(geo, mat);
        scene.add(manualBox);
        transformControl.attach(manualBox);
      }
      manualBox.position.copy(state.manualBoxPos);
      manualBox.scale.copy(state.manualBoxScale);
    } else if (manualBox) {
      transformControl.detach();
      scene.remove(manualBox);
      manualBox = null;
    }

    // Восстанавливаем сидения
    seats.forEach(s => scene.remove(s.mesh));
    seats = [];
    if (state.seats) {
      state.seats.forEach(s => {
        const seatMesh = createSeatMesh();
        seatMesh.position.copy(s.position);
        scene.add(seatMesh);
        seats.push({ mesh: seatMesh, boneName: s.boneName });
      });
    }
    
    updateJSON();
  }

  function createSeatMesh() {
    const geo = new THREE.SphereGeometry(0.3, 16, 16);
    const mat = new THREE.MeshStandardMaterial({ 
      color: COLOR_SEAT, 
      emissive: 0x003355
    });
    return new THREE.Mesh(geo, mat);
  }

  function addSeat(position, boneName) {
    saveHistory();
    const seatMesh = createSeatMesh();
    seatMesh.position.copy(position);
    scene.add(seatMesh);
    seats.push({ mesh: seatMesh, boneName });
    seats = seats; // trigger reactivity
    updateJSON();
  }

  function removeSeat(index) {
    saveHistory();
    scene.remove(seats[index].mesh);
    seats.splice(index, 1);
    seats = seats;
    updateJSON();
  }

  function clearSeats() {
    saveHistory();
    seats.forEach(s => scene.remove(s.mesh));
    seats = [];
    updateJSON();
  }

  function initThreeJS() {
    scene = new THREE.Scene();
    scene.background = new THREE.Color(0x1a1a2e);

    camera = new THREE.PerspectiveCamera(45, 1, 0.1, 2000);
    camera.position.set(40, 40, 40);

    renderer = new THREE.WebGLRenderer({ antialias: true });
    renderer.setPixelRatio(window.devicePixelRatio);
    renderer.shadowMap.enabled = true;
    renderer.shadowMap.type = THREE.PCFSoftShadowMap;
    renderer.toneMapping = THREE.ACESFilmicToneMapping;
    renderer.toneMappingExposure = 1.2;
    if (canvasContainer) {
      renderer.setSize(canvasContainer.clientWidth, canvasContainer.clientHeight);
      canvasContainer.appendChild(renderer.domElement);
    }

    // Приятное освещение
    const amb = new THREE.AmbientLight(0x404060, 0.5);
    scene.add(amb);
    
    // Основной свет (теплый)
    const mainLight = new THREE.DirectionalLight(0xfff0dd, 1.2);
    mainLight.position.set(30, 50, 30);
    mainLight.castShadow = true;
    mainLight.shadow.mapSize.width = 2048;
    mainLight.shadow.mapSize.height = 2048;
    scene.add(mainLight);
    
    // Заполняющий свет (холодный)
    const fillLight = new THREE.DirectionalLight(0x8888ff, 0.4);
    fillLight.position.set(-20, 30, -20);
    scene.add(fillLight);
    
    // Подсветка снизу
    const rimLight = new THREE.DirectionalLight(0xff8844, 0.3);
    rimLight.position.set(0, -20, 0);
    scene.add(rimLight);

    // Hemisphere light для мягкости
    const hemiLight = new THREE.HemisphereLight(0x87ceeb, 0x362312, 0.3);
    scene.add(hemiLight);

    const grid = new THREE.GridHelper(200, 200, 0x3a3a5a, 0x2a2a3a);
    scene.add(grid);
    const axes = new THREE.AxesHelper(5);
    scene.add(axes);

    const planeGeo = new THREE.PlaneGeometry(2000, 2000);
    const planeMat = new THREE.MeshBasicMaterial({ visible: false });
    invisiblePlane = new THREE.Mesh(planeGeo, planeMat);
    invisiblePlane.rotation.x = -Math.PI / 2;
    scene.add(invisiblePlane);

    ghostBox = new THREE.Mesh(
      new THREE.BoxGeometry(1, 1, 1),
      new THREE.MeshBasicMaterial({ color: 0xff0000, transparent: true, opacity: 0.4, depthTest: false })
    );
    ghostBox.visible = false;
    scene.add(ghostBox);

    controls = new OrbitControls(camera, renderer.domElement);
    controls.enableDamping = false;
    controls.zoomSpeed = 3;
    controls.rotateSpeed = 1;
    controls.panSpeed = 1.5;
    controls.mouseButtons = {
      LEFT: THREE.MOUSE.ROTATE,
      MIDDLE: THREE.MOUSE.DOLLY,
      RIGHT: THREE.MOUSE.PAN
    };

    transformControl = new TransformControls(camera, renderer.domElement);
    transformControl.addEventListener('change', () => { 
      if(manualBox || selectedSeatIndex >= 0) updateJSON(); 
    });
    transformControl.addEventListener('dragging-changed', (e) => { 
      controls.enabled = !e.value;
      // Сохраняем историю ПЕРЕД началом перетаскивания
      if (e.value && (manualBox || selectedSeatIndex >= 0)) saveHistory();
    });
    scene.add(transformControl);

    raycaster = new THREE.Raycaster();
    mouse = new THREE.Vector2();

    renderer.domElement.addEventListener('pointerdown', onPointerDown);
    renderer.domElement.addEventListener('pointermove', onPointerMove);
    renderer.domElement.addEventListener('pointerup', onPointerUp);
    renderer.domElement.addEventListener('contextmenu', (e) => e.preventDefault());

    animate();
    setTimeout(onWindowResize, 50);
  }

  // FPS камера переменные
  let isMouseLocked = false;
  let yaw = 0;
  let pitch = 0;

  function syncYawPitch() {
    // Синхронизируем yaw/pitch с текущим направлением камеры
    const dir = new THREE.Vector3();
    camera.getWorldDirection(dir);
    yaw = Math.atan2(-dir.x, -dir.z);
    pitch = Math.asin(dir.y);
  }

  function setupWASDTarget(length = 1) {
    // Как в Blockbench - ставим target близко перед камерой
    const pos = new THREE.Vector3().copy(camera.position);
    const dir = camera.getWorldDirection(new THREE.Vector3()).normalize().multiplyScalar(length);
    pos.add(dir);
    controls.target.copy(pos);
  }

  function animate() {
    requestAnimationFrame(animate);
    
    // WASD движение (как в Blockbench)
    const hasMovement = keys.w || keys.s || keys.a || keys.d || keys.space || keys.shift;
    
    if (hasMovement && cameraMode) {
      // Ставим target близко перед камерой
      setupWASDTarget(1);
      
      // Скорость: Ctrl = быстро
      let speed = moveSpeed;
      if (keys.ctrl) speed = moveSpeed * 3;
      
      const movement = new THREE.Vector3(0, 0, 0);
      
      // Собираем направление движения
      if (keys.w) movement.z -= 1;
      if (keys.s) movement.z += 1;
      if (keys.a) movement.x -= 1;
      if (keys.d) movement.x += 1;
      if (keys.space) movement.y += 1;
      if (keys.shift) movement.y -= 1;
      
      if (movement.length() > 0) {
        // Поворачиваем по направлению камеры (только yaw, горизонтально)
        const dir = camera.getWorldDirection(new THREE.Vector3()).normalize();
        const angle = Math.atan2(-dir.x, -dir.z);
        movement.applyAxisAngle(new THREE.Vector3(0, 1, 0), angle);
        
        movement.multiplyScalar(speed);
        camera.position.add(movement);
        controls.target.add(movement);
      }
    }
    
    controls.update();
    renderer.render(scene, camera);
  }

  function onWindowResize() {
    if (!canvasContainer) return;
    const w = canvasContainer.clientWidth;
    const h = canvasContainer.clientHeight;
    camera.aspect = w / h;
    camera.updateProjectionMatrix();
    renderer.setSize(w, h);
  }

  function onKeyDown(e) {
    // WASD + русская раскладка ЦФЫВ
    const key = e.key.toLowerCase();
    if (key === 'w' || key === 'ц') keys.w = true;
    if (key === 'a' || key === 'ф') keys.a = true;
    if (key === 's' || key === 'ы') keys.s = true;
    if (key === 'd' || key === 'в') keys.d = true;
    if (e.key === 'Shift') keys.shift = true;
    if (e.key === ' ') { keys.space = true; e.preventDefault(); }
    if (e.key === 'Control') keys.ctrl = true;

    // Ctrl+Z
    if ((e.ctrlKey || e.metaKey) && (e.key === 'z' || e.key === 'Z' || e.keyCode === 90)) {
      e.preventDefault();
      e.stopPropagation();
      undo();
      return;
    }
    
    switch(key) {
      case 't': setEditMode('translate'); break;
      case 'r': setEditMode('scale'); break;
      case 'delete': deleteManualBox(); break;
      case 'escape': 
        cancelDrawing(); 
        selectedSeatIndex = -1;
        if (transformControl.object && seats.some(s => s.mesh === transformControl.object)) {
          transformControl.detach();
        }
        break;
    }
  }

  function onKeyUp(e) {
    const key = e.key.toLowerCase();
    if (key === 'w' || key === 'ц') keys.w = false;
    if (key === 'a' || key === 'ф') keys.a = false;
    if (key === 's' || key === 'ы') keys.s = false;
    if (key === 'd' || key === 'в') keys.d = false;
    if (e.key === 'Shift') keys.shift = false;
    if (e.key === ' ') keys.space = false;
    if (e.key === 'Control') keys.ctrl = false;
  }

  function setEditMode(mode) {
    editMode = mode;
    if (manualBox) transformControl.setMode(mode);
  }

  function startDrawing() {
    if (manualBox) deleteManualBox();
    creationPhase = 1;
    document.body.style.cursor = 'crosshair';
  }

  function cancelDrawing() {
    creationPhase = 0;
    ghostBox.visible = false;
    document.body.style.cursor = 'default';
  }

  function deleteManualBox(save = true) {
    if (save) saveHistory();
    if (manualBox) {
      transformControl.detach();
      scene.remove(manualBox);
      manualBox = null;
    }
    updateJSON();
  }

  function createManualBox(position, size) {
    saveHistory();
    if (manualBox) deleteManualBox(false);

    const geo = new THREE.BoxGeometry(1, 1, 1);
    const mat = new THREE.MeshStandardMaterial({ 
      color: 0xff4444, 
      transparent: true, 
      opacity: 0.6,
      emissive: 0x331111
    });
    manualBox = new THREE.Mesh(geo, mat);
    
    manualBox.position.copy(position);
    manualBox.scale.copy(size);
    manualBox.scale.max(new THREE.Vector3(0.1, 0.1, 0.1));

    scene.add(manualBox);
    transformControl.attach(manualBox);
    transformControl.setMode(editMode);
    updateJSON();
  }

  function getIntersect(e, includeSeatMeshes = false) {
    const rect = renderer.domElement.getBoundingClientRect();
    mouse.x = ((e.clientX - rect.left) / rect.width) * 2 - 1;
    mouse.y = -((e.clientY - rect.top) / rect.height) * 2 + 1;
    raycaster.setFromCamera(mouse, camera);
    const seatMeshes = includeSeatMeshes ? seats.map(s => s.mesh) : [];
    const objects = [...seatMeshes, ...modelMeshes, invisiblePlane];
    const intersects = raycaster.intersectObjects(objects);
    return intersects.length > 0 ? intersects[0] : null;
  }

  let pointerDownPos = { x: 0, y: 0 };

  function onPointerDown(e) {
    pointerDownPos = { x: e.clientX, y: e.clientY };
    isDragging = false;
    
    if (transformControl.dragging || e.button !== 0) return;
    if (cameraMode) return; // В режиме камеры не выделяем
  }

  function onPointerUp(e) {
    // Проверяем было ли это перетаскивание (движение > 5px)
    const dx = Math.abs(e.clientX - pointerDownPos.x);
    const dy = Math.abs(e.clientY - pointerDownPos.y);
    if (dx > 5 || dy > 5) {
      isDragging = true;
      return;
    }

    if (transformControl.dragging || e.button !== 0) return;
    if (cameraMode) return;

    // Проверяем клик по сидению (приоритет)
    const seatIntersect = getIntersect(e, true);
    if (seatIntersect) {
      const seatIdx = seats.findIndex(s => s.mesh === seatIntersect.object);
      if (seatIdx !== -1) {
        selectSeat(seatIdx);
        return;
      }
    }

    const intersect = getIntersect(e);
    if (!intersect) return;

    // Режим добавления сидения
    if (seatMode) {
      if (intersect.object && intersect.object.userData && intersect.object.userData.boneName) {
        addSeat(intersect.point, intersect.object.userData.boneName);
      }
      return;
    }

    if (creationPhase === 1) {
      startPoint.copy(intersect.point);
      ghostBox.position.copy(intersect.point);
      ghostBox.scale.set(0.1, 0.1, 0.1);
      ghostBox.visible = true;
      creationPhase = 2;
      return;
    } else if (creationPhase === 2) {
      createManualBox(ghostBox.position, ghostBox.scale);
      ghostBox.visible = false;
      creationPhase = 0;
      document.body.style.cursor = 'default';
      return;
    }

    if (intersect.object && intersect.object.userData && intersect.object.userData.boneName) {
      saveHistory();
      if (selectionMode === 'cube') {
        toggleMeshSelection(intersect.object);
      } else if (selectionMode === 'bone') {
        toggleBoneSelection(intersect.object.userData.boneName);
      }
    }
  }

  function selectSeat(index) {
    selectedSeatIndex = index;
    // Отсоединяем от manualBox если был
    if (manualBox && transformControl.object === manualBox) {
      // оставляем
    } else {
      transformControl.detach();
    }
    transformControl.attach(seats[index].mesh);
    transformControl.setMode('translate');
  }

  function viewFromSeat(index) {
    if (index < 0 || index >= seats.length) return;
    const seat = seats[index];
    const pos = seat.mesh.position.clone();
    // Высота глаз игрока ~1.2 блока
    pos.y += 1.2;
    camera.position.copy(pos);
    // Игрок смотрит на +Z в Minecraft, но в Three.js у нас инверсия Z, поэтому -Z
    controls.target.set(pos.x, pos.y, pos.z - 5);
    controls.update();
  }

  function onPointerMove(e) {
    if (creationPhase !== 2) return;
    const intersect = getIntersect(e);
    if (!intersect) return;

    const min = new THREE.Vector3().minVectors(startPoint, intersect.point);
    const max = new THREE.Vector3().maxVectors(startPoint, intersect.point);
    const size = new THREE.Vector3().subVectors(max, min);
    size.max(new THREE.Vector3(0.1, 0.1, 0.1));
    const center = new THREE.Vector3().addVectors(min, max).multiplyScalar(0.5);

    ghostBox.position.copy(center);
    ghostBox.scale.copy(size);
  }

  function toggleMeshSelection(mesh) {
    const meshId = mesh.uuid;
    const boneColor = boneColorMap[mesh.userData?.boneName] || 0x888888;
    
    if (selectedMeshes.has(meshId)) {
      selectedMeshes.delete(meshId);
      mesh.material.color.setHex(boneColor);
      mesh.material.emissive.setHex(0x000000);
    } else {
      selectedMeshes.add(meshId);
      mesh.material.color.setHex(COLOR_SELECTED);
      mesh.material.emissive.setHex(0x003300);
    }
    
    selectedMeshes = selectedMeshes;
    updateJSON();
  }

  function toggleBoneSelection(boneName) {
    const boneMeshes = modelMeshes.filter(m => m.userData.boneName === boneName);
    const allSelected = boneMeshes.every(m => selectedMeshes.has(m.uuid));
    const boneColor = boneColorMap[boneName] || 0x888888;
    
    boneMeshes.forEach(mesh => {
      if (allSelected) {
        selectedMeshes.delete(mesh.uuid);
        mesh.material.color.setHex(boneColor);
        mesh.material.emissive.setHex(0x000000);
      } else {
        selectedMeshes.add(mesh.uuid);
        mesh.material.color.setHex(COLOR_SELECTED);
        mesh.material.emissive.setHex(0x003300);
      }
    });
    
    selectedMeshes = selectedMeshes;
    updateJSON();
  }

  function clearSelection() {
    saveHistory();
    selectedMeshes.forEach(meshId => {
      const mesh = modelMeshes.find(m => m.uuid === meshId);
      if (mesh) {
        const boneColor = boneColorMap[mesh.userData?.boneName] || 0x888888;
        mesh.material.color.setHex(boneColor);
        mesh.material.emissive.setHex(0x000000);
      }
    });
    selectedMeshes.clear();
    selectedMeshes = selectedMeshes;
    updateJSON();
  }

  function selectAll() {
    saveHistory();
    modelMeshes.forEach(mesh => {
      selectedMeshes.add(mesh.uuid);
      mesh.material.color.setHex(COLOR_SELECTED);
      mesh.material.emissive.setHex(0x003300);
    });
    selectedMeshes = selectedMeshes;
    updateJSON();
  }

  function updateJSON() {
    const obbArray = [];

    selectedMeshes.forEach(meshId => {
      const mesh = modelMeshes.find(m => m.uuid === meshId);
      if (!mesh || !mesh.userData) return;

      const { boneName, originalOrigin, originalSize } = mesh.userData;

      // Оригинальная формула: Size = originalSize / 32
      const size = [
        toFloat(originalSize[0] / 32),
        toFloat(originalSize[1] / 32),
        toFloat(originalSize[2] / 32)
      ];

      // Position = origin / 16 с инверсией X,Z (БЕЗ добавления size/2)
      const position = [
        toFloat((originalOrigin[0] / 16) * -1),
        toFloat(originalOrigin[1] / 16),
        toFloat((originalOrigin[2] / 16) * -1)
      ];

      obbArray.push({ "Size": size, "Position": position, "Part": boneName });
    });

    if (manualBox) {
      const pos = manualBox.position;
      const scale = manualBox.scale;

      obbArray.push({
        "Size": [toFloat(scale.x / 2), toFloat(scale.y / 2), toFloat(scale.z / 2)],
        "Position": [toFloat(pos.x * -1), toFloat(pos.y), toFloat(pos.z * -1)],
        "Part": "Manual"
      });
    }

    // Генерация Seats (игрок смотрит на +Z)
    const seatsArray = seats.map(s => {
      const pos = s.mesh.position;
      return {
        "HidePassenger": true,
        "BanHand": true,
        "Transform": s.boneName,
        "Position": [
          toFloat(pos.x * -1),
          toFloat(pos.y),
          toFloat(pos.z * -1)
        ]
      };
    });

    if (obbArray.length === 0 && seatsArray.length === 0) {
      outputText = '';
      return;
    }

    let result = {};
    if (obbArray.length > 0) result["OBB"] = obbArray;
    if (seatsArray.length > 0) result["Seats"] = seatsArray;
    
    // Формат с отступами внутри объектов
    let parts = [];
    if (result["OBB"]) {
      const obbItems = result["OBB"].map(o => 
        `{\n  "Size": [${o.Size.join(', ')}],\n  "Position": [${o.Position.join(', ')}],\n  "Part": "${o.Part}"\n}`
      );
      parts.push(`"OBB": [\n${obbItems.join(',\n')}\n]`);
    }
    if (result["Seats"]) {
      const seatsItems = result["Seats"].map(s => 
        `{\n  "HidePassenger": ${s.HidePassenger},\n  "BanHand": ${s.BanHand},\n  "Transform": "${s.Transform}",\n  "Position": [${s.Position.join(', ')}]\n}`
      );
      parts.push(`"Seats": [\n${seatsItems.join(',\n')}\n]`);
    }
    outputText = parts.join(',\n');
  }

  function handleModelUpload(e) {
    const file = e.target.files[0];
    if(!file) return;
    const reader = new FileReader();
    reader.onload = (ev) => {
      try {
        modelData = JSON.parse(ev.target.result);
        buildModel();
      } catch(err) { 
        showError("Ошибка парсинга JSON"); 
      }
    };
    reader.readAsText(file);
  }

  function buildModel() {
    modelMeshes.forEach(m => { 
      m.geometry.dispose(); 
      m.material.dispose(); 
      scene.remove(m); 
    });
    modelMeshes = [];
    selectedMeshes.clear();
    history = [];
    if (manualBox) deleteManualBox(false);
    outputText = '';
    errorMessage = '';

    const geo = modelData["minecraft:geometry"] ? modelData["minecraft:geometry"][0] : null;
    if(!geo || !geo.bones) {
      showError("Не найден minecraft:geometry или bones");
      return;
    }

    bonesList = geo.bones.map(b => b.name);

    // Назначаем цвета костям
    boneColorMap = {};
    bonesList.forEach((name, i) => {
      boneColorMap[name] = BONE_COLORS[i % BONE_COLORS.length];
    });

    const boneMap = {};
    geo.bones.forEach(b => {
      const g = new THREE.Group();
      boneMap[b.name] = { meta: b, group: g };
    });

    geo.bones.forEach(b => {
      const node = boneMap[b.name];
      if(b.parent && boneMap[b.parent]) {
        boneMap[b.parent].group.add(node.group);
        const pp = boneMap[b.parent].meta.pivot || [0,0,0];
        const cp = b.pivot || [0,0,0];
        node.group.position.set(-(cp[0]-pp[0])/16, (cp[1]-pp[1])/16, (cp[2]-pp[2])/16);
      } else {
        scene.add(node.group);
        const cp = b.pivot || [0,0,0];
        node.group.position.set(-cp[0]/16, cp[1]/16, cp[2]/16);
      }
      if(b.rotation) {
        node.group.rotation.set(-b.rotation[0]*DEG2RAD, -b.rotation[1]*DEG2RAD, b.rotation[2]*DEG2RAD, 'ZYX');
      }

      if(b.cubes) {
        b.cubes.forEach(c => {
          const sz = c.size; 
          const og = c.origin; 
          const inf = c.inflate || 0;
          
          const geometry = new THREE.BoxGeometry(
            (sz[0]+inf*2)/16, 
            (sz[1]+inf*2)/16, 
            (sz[2]+inf*2)/16
          );

          const boneColor = boneColorMap[b.name] || 0x888888;
          const material = new THREE.MeshStandardMaterial({ 
            color: boneColor,
            roughness: 0.7,
            metalness: 0.1,
            side: THREE.DoubleSide,
            emissive: 0x000000
          });
          
          const mesh = new THREE.Mesh(geometry, material);
          mesh.castShadow = true;
          mesh.receiveShadow = true;
          
          const center = [og[0]+sz[0]/2, og[1]+sz[1]/2, og[2]+sz[2]/2];
          const bp = b.pivot || [0,0,0];
          const pg = new THREE.Group();
          node.group.add(pg);

          if(c.rotation) {
            const cp = c.pivot || center;
            pg.position.set(-(cp[0]-bp[0])/16, (cp[1]-bp[1])/16, (cp[2]-bp[2])/16);
            pg.rotation.set(-c.rotation[0]*DEG2RAD, -c.rotation[1]*DEG2RAD, c.rotation[2]*DEG2RAD, 'ZYX');
            mesh.position.set(-(center[0]-cp[0])/16, (center[1]-cp[1])/16, (center[2]-cp[2])/16);
          } else {
            pg.position.set(-(center[0]-bp[0])/16, (center[1]-bp[1])/16, (center[2]-bp[2])/16);
          }
          
          mesh.userData = { 
            boneName: b.name,
            originalOrigin: [...og],
            originalSize: [...sz]
          };
          
          pg.add(mesh);
          modelMeshes.push(mesh);
        });
      }
    });

    if (modelMeshes.length > 0) centerCamera();
  }

  function centerCamera() {
    const box = new THREE.Box3();
    modelMeshes.forEach(mesh => box.expandByObject(mesh));

    const center = new THREE.Vector3();
    box.getCenter(center);
    const size = new THREE.Vector3();
    box.getSize(size);
    const maxDim = Math.max(size.x, size.y, size.z);
    const fov = camera.fov * (Math.PI / 180);
    let cameraZ = Math.abs(maxDim / 2 / Math.tan(fov / 2)) * 2;

    camera.position.set(center.x + cameraZ, center.y + cameraZ * 0.5, center.z + cameraZ);
    camera.lookAt(center);
    controls.target.copy(center);
    controls.update();
  }

  function toFloat(n) { return parseFloat(n.toFixed(4)); }
  function showError(msg) { errorMessage = msg; }
  function copyText() { navigator.clipboard.writeText(outputText); }
</script>

<main>
  <div class="app-layout">
    <div class="viewport-area">
      <div class="toolbar">
        <div class="logo">
          <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"/>
            <polyline points="3.27 6.96 12 12.01 20.73 6.96"/>
            <line x1="12" y1="22.08" x2="12" y2="12"/>
          </svg>
          <span>{t.title}</span>
        </div>
        <div class="toolbar-right">
          <button class="btn-icon-toolbar" on:click={() => showTutorial = true} title={t.tutorial}>
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><path d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3"/><line x1="12" y1="17" x2="12.01" y2="17"/></svg>
          </button>
          <a href="https://github.com/denys-shatin/OBB-Lab" target="_blank" rel="noopener noreferrer" class="btn-icon-toolbar" title="GitHub Repository">
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22"/></svg>
          </a>
          <div class="lang-switcher">
            <button class:active={currentLang === 'ru'} on:click={() => setLang('ru')}>RU</button>
            <button class:active={currentLang === 'en'} on:click={() => setLang('en')}>EN</button>
            <button class:active={currentLang === 'ja'} on:click={() => setLang('ja')}>日本語</button>
          </div>
          <span class="hint">{t.undo}</span>
          <label class="btn-upload">
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
              <polyline points="17 8 12 3 7 8"/>
              <line x1="12" y1="3" x2="12" y2="15"/>
            </svg>
            {t.upload}
            <input type="file" accept=".json" on:change={handleModelUpload} />
          </label>
        </div>
      </div>
      <div class="canvas-wrapper" bind:this={canvasContainer}></div>
      <div class="status-bar">
        {#if cameraMode}
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><polygon points="16.24 7.76 14.12 14.12 7.76 16.24 9.88 9.88 16.24 7.76"/></svg>
          {t.wasdMode}
        {:else if seatMode}
          <span class="pulse">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M4 18v-5a3 3 0 0 1 3-3h10a3 3 0 0 1 3 3v5"/><rect x="5" y="3" width="14" height="7" rx="2"/></svg>
            {t.clickForSeat}
          </span>
        {:else if creationPhase > 0}
          <span class="pulse">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 19l7-7 3 3-7 7-3-3z"/><path d="M18 13l-1.5-7.5L2 2l3.5 14.5L13 18l5-5z"/></svg>
            {t.clickToDraw}
          </span>
        {:else if manualBox}
          �️ Ручной бокс | T - Move | R - Scale | Delete - удалить
        {:else if modelMeshes.length > 0}
          🖱️ Клик = выделить | Режим: <b style="color:#ff9800">{selectionMode === 'cube' ? 'Куб' : 'Кость'}</b> | Выделено: <b style="color:#00ff00">{selectedMeshes.size}</b>
        {:else}
          Загрузи JSON файл модели
        {/if}
      </div>
    </div>

    <div class="sidebar">
      {#if errorMessage} <div class="error-box">⚠️ {errorMessage}</div> {/if}

      <div class="panel-section">
        <h3>Камера</h3>
        <div class="mode-toggle">
          <button class="mode-btn" class:active={!cameraMode} on:click={() => { cameraMode = false; controls.enableZoom = true; }}>
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M3 3h7v7H3z"/><path d="M14 3h7v7h-7z"/><path d="M14 14h7v7h-7z"/><path d="M3 14h7v7H3z"/></svg>
            {t.select}
          </button>
          <button class="mode-btn" class:active={cameraMode} on:click={() => { cameraMode = true; controls.enableZoom = false; setupWASDTarget(1); }}>
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><polygon points="16.24 7.76 14.12 14.12 7.76 16.24 9.88 9.88 16.24 7.76"/></svg>
            {t.wasd}
          </button>
        </div>
        {#if cameraMode}
          <div class="speed-slider">
            <label for="speed-input">{t.speed}: {moveSpeed.toFixed(2)}</label>
            <input id="speed-input" type="range" min="0.02" max="0.5" step="0.01" bind:value={moveSpeed} />
          </div>
        {/if}
      </div>

      {#if !cameraMode}
      <div class="panel-section">
        <h3>{t.selection}</h3>
        <div class="mode-toggle">
          <button class="mode-btn" class:active={selectionMode === 'cube'} on:click={() => selectionMode = 'cube'}>
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"/></svg>
            {t.cube}
          </button>
          <button class="mode-btn" class:active={selectionMode === 'bone'} on:click={() => selectionMode = 'bone'}>
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M17 10c.7-.7 1.69 0 2.5 0a2.5 2.5 0 1 0 0-5 .5.5 0 0 1-.5-.5 2.5 2.5 0 1 0-5 0c0 .81.7 1.8 0 2.5l-7 7c-.7.7-1.69 0-2.5 0a2.5 2.5 0 0 0 0 5c.28 0 .5.22.5.5a2.5 2.5 0 1 0 5 0c0-.81-.7-1.8 0-2.5Z"/></svg>
            {t.bone}
          </button>
        </div>
      </div>
      {/if}

      <div class="panel-section">
        <h3>{t.tools}</h3>
        {#if modelMeshes.length > 0}
          <div class="btn-group">
            <button class="btn glass" on:click={selectAll}>
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="20 6 9 17 4 12"/></svg>
              {t.all}
            </button>
            <button class="btn glass" on:click={clearSelection}>
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>
              {t.reset}
            </button>
          </div>
        {/if}
        
        <button class="btn blue" on:click={startDrawing} disabled={creationPhase !== 0 || seatMode || cameraMode} style={cameraMode ? 'display:none' : ''}>
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 19l7-7 3 3-7 7-3-3z"/><path d="M18 13l-1.5-7.5L2 2l3.5 14.5L13 18l5-5z"/></svg>
          {creationPhase === 0 ? t.drawBox : t.drawing}
        </button>

        <button class="btn purple" class:active={seatMode} on:click={() => { seatMode = !seatMode; if(seatMode) creationPhase = 0; }} style={cameraMode ? 'display:none' : ''}>
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M4 18v-5a3 3 0 0 1 3-3h10a3 3 0 0 1 3 3v5"/><rect x="5" y="3" width="14" height="7" rx="2"/></svg>
          {seatMode ? t.disableSeat : t.addSeat}
        </button>

        {#if manualBox}
          <div class="card">
            <div class="card-header">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"/></svg>
              {t.manualBox}
            </div>
            <div class="btn-row">
              <button class="btn small glass" class:active={editMode === 'translate'} on:click={() => setEditMode('translate')}>Move</button>
              <button class="btn small glass" class:active={editMode === 'scale'} on:click={() => setEditMode('scale')}>Scale</button>
            </div>
            <button class="btn small red" on:click={() => deleteManualBox()}>
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="3 6 5 6 21 6"/><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/></svg>
              {t.delete}
            </button>
          </div>
        {/if}

        {#if seats.length > 0}
          <div class="card purple-border">
            <div class="card-header purple">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M4 18v-5a3 3 0 0 1 3-3h10a3 3 0 0 1 3 3v5"/><rect x="5" y="3" width="14" height="7" rx="2"/></svg>
              {t.seats}: {seats.length}
              <button class="btn-icon" on:click={clearSeats}>
                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="3 6 5 6 21 6"/><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/></svg>
              </button>
            </div>
            {#each seats as seat, i}
              <button class="seat-item" class:selected={selectedSeatIndex === i} on:click={() => selectSeat(i)}>
                <span>{i + 1}. {seat.boneName}</span>
                <div class="seat-actions">
                  <span class="btn-icon" role="button" tabindex="0" on:click|stopPropagation={() => viewFromSeat(i)} on:keydown|stopPropagation={(e) => e.key === 'Enter' && viewFromSeat(i)} title="Вид от игрока">
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"/><circle cx="12" cy="12" r="3"/></svg>
                  </span>
                  <span class="btn-icon red" role="button" tabindex="0" on:click|stopPropagation={() => removeSeat(i)} on:keydown|stopPropagation={(e) => e.key === 'Enter' && removeSeat(i)}>
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>
                  </span>
                </div>
              </button>
            {/each}
          </div>
        {/if}
      </div>

      {#if selectedMeshes.size > 0 || history.length > 0}
        <div class="panel-section">
          <div class="info-row">
            {#if selectedMeshes.size > 0}
              <div class="info-badge green">
                <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="20 6 9 17 4 12"/></svg>
                {selectedMeshes.size}
              </div>
            {/if}
            {#if history.length > 0}
              <div class="info-badge blue">
                <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="1 4 1 10 7 10"/><path d="M3.51 15a9 9 0 1 0 2.13-9.36L1 10"/></svg>
                {history.length}
              </div>
            {/if}
          </div>
        </div>
      {/if}

      <div class="panel-section grow">
        <h3>{t.result}</h3>
        <textarea readonly value={outputText} placeholder="Выдели кубы или нарисуй бокс..."></textarea>
      </div>
      
      <button class="btn primary" disabled={!outputText} on:click={copyText}>
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"/><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/></svg>
        {t.copy}
      </button>
    </div>
  </div>
</main>

{#if showTutorial}
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div class="modal-overlay" on:click={() => showTutorial = false} role="dialog" aria-modal="true" aria-labelledby="modal-title">
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div class="modal-content" on:click|stopPropagation>
      <div class="modal-header" id="modal-title">{t.tutorialTitle}</div>
      {#each t.tutorialItems as item}
        <div class="tutorial-item">
          <h4>{item.title}</h4>
          <p>{item.desc}</p>
        </div>
      {/each}
      <button class="modal-close" on:click={() => showTutorial = false}>{t.close}</button>
    </div>
  </div>
{/if}

<style>
  :global(body) { 
    margin: 0; 
    background: #000; 
    color: #f5f5f7; 
    font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Display', 'Segoe UI', sans-serif; 
    overflow: hidden; 
    user-select: none;
    -webkit-font-smoothing: antialiased;
  }
  
  .app-layout { display: flex; width: 100vw; height: 100vh; }
  .viewport-area { flex: 1; display: flex; flex-direction: column; position: relative; background: #0a0a0f; }
  
  .toolbar { 
    position: absolute; top: 0; left: 0; right: 0; 
    padding: 14px 24px; 
    background: rgba(0,0,0,0.7); 
    display: flex; justify-content: space-between; align-items: center; 
    z-index: 10; 
    border-bottom: 1px solid rgba(255,255,255,0.06);
    backdrop-filter: blur(20px);
    -webkit-backdrop-filter: blur(20px);
  }
  .toolbar-right { display: flex; align-items: center; gap: 20px; }
  .hint { color: #6e6e73; font-size: 0.85rem; }
  
  .logo { 
    display: flex; 
    align-items: center; 
    gap: 10px; 
    font-weight: 600; 
    font-size: 1.1rem;
    letter-spacing: -0.01em;
    color: #f5f5f7;
  }
  .logo svg { color: #2997ff; }
  
  .btn-upload { 
    display: flex;
    align-items: center;
    gap: 8px;
    background: rgba(255,255,255,0.1);
    border: 1px solid rgba(255,255,255,0.2);
    color: #fff; 
    padding: 10px 20px; 
    border-radius: 980px; 
    cursor: pointer; 
    font-weight: 500;
    font-size: 0.95rem; 
    transition: all 0.2s;
    backdrop-filter: blur(10px);
  }
  .btn-upload:hover { 
    background: rgba(255,255,255,0.15);
    border-color: rgba(255,255,255,0.3);
    transform: scale(1.02);
  }
  input[type="file"] { display: none; }
  
  .canvas-wrapper { flex: 1; width: 100%; height: 100%; }
  
  .status-bar { 
    padding: 12px 24px; 
    background: rgba(0,0,0,0.8); 
    color: #86868b; 
    font-size: 0.9rem; 
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    border-top: 1px solid rgba(255,255,255,0.06);
    backdrop-filter: blur(10px);
  }
  .status-bar svg { opacity: 0.7; }
  .pulse { color: #30d158; animation: pulse 1.5s ease-in-out infinite; display: flex; align-items: center; gap: 8px; }
  @keyframes pulse { 0%,100% { opacity: 1; } 50% { opacity: 0.5; } }
  
  .sidebar { 
    width: 320px; 
    background: rgba(0,0,0,0.95);
    border-left: 1px solid rgba(255,255,255,0.06);
    display: flex; flex-direction: column; 
    padding: 16px; 
    gap: 12px;
    backdrop-filter: blur(20px);
  }
  
  .panel-section { }
  .panel-section.grow { flex: 1; display: flex; flex-direction: column; }
  
  h3 { 
    margin: 0 0 10px 0; 
    font-size: 0.75rem; 
    color: #6e6e73; 
    text-transform: uppercase; 
    letter-spacing: 0.5px;
    font-weight: 600;
  }

  .mode-toggle { 
    display: flex; 
    gap: 4px; 
    background: rgba(255,255,255,0.03); 
    padding: 4px; 
    border-radius: 12px;
    border: 1px solid rgba(255,255,255,0.06);
  }
  .mode-btn { 
    flex: 1; 
    padding: 10px 12px; 
    border: none; 
    background: transparent; 
    color: #86868b; 
    cursor: pointer; 
    border-radius: 10px; 
    font-weight: 500; 
    font-size: 0.9rem;
    transition: all 0.2s;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
  }
  .mode-btn:hover { background: rgba(255,255,255,0.05); color: #f5f5f7; }
  .mode-btn.active { background: rgba(255,255,255,0.1); color: #fff; }
  .mode-btn svg { width: 16px; height: 16px; }

  .speed-slider { margin-top: 12px; }
  .speed-slider label { display: block; font-size: 0.8rem; color: #6e6e73; margin-bottom: 8px; }
  .speed-slider input[type="range"] { 
    width: 100%; 
    accent-color: #2997ff;
    height: 4px;
    border-radius: 2px;
  }
  
  .btn-group { display: flex; gap: 8px; margin-bottom: 10px; }
  .btn-group .btn { flex: 1; }
  
  .info-row { display: flex; gap: 8px; }
  .info-badge { 
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 8px 12px; 
    border-radius: 8px; 
    font-size: 0.85rem;
    font-weight: 500;
    background: rgba(255,255,255,0.03);
    border: 1px solid rgba(255,255,255,0.06);
  }
  .info-badge.green { color: #30d158; border-color: rgba(48,209,88,0.3); }
  .info-badge.blue { color: #2997ff; border-color: rgba(41,151,255,0.3); }
  
  textarea { 
    flex: 1; 
    background: rgba(255,255,255,0.03); 
    color: #ffd60a; 
    border: 1px solid rgba(255,255,255,0.06); 
    padding: 12px; 
    resize: none; 
    font-family: 'SF Mono', 'Consolas', monospace; 
    font-size: 0.8rem; 
    border-radius: 12px; 
    line-height: 1.5;
  }
  textarea::placeholder { color: #3a3a3c; }
  textarea:focus { outline: none; border-color: rgba(41,151,255,0.5); }
  
  .btn { 
    border: none; 
    cursor: pointer; 
    font-weight: 500; 
    border-radius: 12px; 
    transition: all 0.2s; 
    padding: 12px 16px;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    font-size: 0.9rem;
  }
  
  .btn.primary { 
    background: rgba(255,255,255,0.1);
    border: 1px solid rgba(255,255,255,0.2);
    color: #fff;
    backdrop-filter: blur(10px);
  }
  .btn.primary:hover { 
    background: rgba(255,255,255,0.15);
    border-color: rgba(255,255,255,0.3);
    transform: scale(1.01);
  }
  .btn.primary:disabled { 
    background: rgba(255,255,255,0.03); 
    border-color: rgba(255,255,255,0.06);
    color: #3a3a3c; 
    cursor: not-allowed;
    transform: none;
  }
  
  .btn.glass { 
    background: rgba(255,255,255,0.03);
    border: 1px solid rgba(255,255,255,0.06);
    color: #f5f5f7;
  }
  .btn.glass:hover { 
    background: rgba(255,255,255,0.06);
    border-color: rgba(255,255,255,0.1);
  }
  .btn.glass.active { 
    background: rgba(41,151,255,0.15);
    border-color: rgba(41,151,255,0.3);
    color: #2997ff;
  }

  .btn.blue { 
    background: rgba(41,151,255,0.15);
    border: 1px solid rgba(41,151,255,0.3);
    color: #2997ff;
    width: 100%; 
    margin-bottom: 8px;
  }
  .btn.blue:hover { 
    background: rgba(41,151,255,0.2);
    border-color: rgba(41,151,255,0.4);
  }
  .btn.blue:disabled { 
    background: rgba(255,255,255,0.03);
    border-color: rgba(255,255,255,0.06);
    color: #3a3a3c;
  }

  .btn.purple { 
    background: rgba(191,90,242,0.15);
    border: 1px solid rgba(191,90,242,0.3);
    color: #bf5af2;
    width: 100%;
  }
  .btn.purple:hover { 
    background: rgba(191,90,242,0.2);
    border-color: rgba(191,90,242,0.4);
  }
  .btn.purple.active { 
    background: rgba(48,209,88,0.15);
    border-color: rgba(48,209,88,0.3);
    color: #30d158;
  }

  .card {
    margin-top: 10px;
    background: rgba(255,255,255,0.03);
    border: 1px solid rgba(255,255,255,0.06);
    border-radius: 12px;
    padding: 12px;
  }
  .card.purple-border { border-color: rgba(191,90,242,0.3); }
  
  .card-header {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 0.85rem;
    font-weight: 500;
    margin-bottom: 10px;
    color: #f5f5f7;
  }
  .card-header.purple { color: #bf5af2; }
  .card-header .btn-icon { margin-left: auto; }

  .btn-row { display: flex; gap: 6px; margin-bottom: 8px; }
  .btn.small { 
    flex: 1; 
    padding: 8px 12px; 
    font-size: 0.8rem;
  }
  .btn.small.red { 
    background: rgba(255,69,58,0.15);
    border: 1px solid rgba(255,69,58,0.3);
    color: #ff453a;
  }
  .btn.small.red:hover { 
    background: rgba(255,69,58,0.2);
  }

  .seat-item { 
    display: flex; 
    justify-content: space-between; 
    align-items: center; 
    padding: 8px 10px; 
    background: rgba(255,255,255,0.03);
    border: 1px solid transparent;
    border-radius: 8px; 
    margin-bottom: 6px; 
    font-size: 0.85rem; 
    cursor: pointer; 
    transition: all 0.2s;
    color: #86868b;
  }
  .seat-item:hover { 
    background: rgba(255,255,255,0.05);
    border-color: rgba(255,255,255,0.06);
    color: #f5f5f7;
  }
  .seat-item.selected { 
    background: rgba(191,90,242,0.15);
    border-color: rgba(191,90,242,0.3);
    color: #bf5af2;
  }
  .seat-actions { display: flex; gap: 4px; }
  
  .btn-icon { 
    background: transparent;
    border: none;
    color: #6e6e73;
    padding: 4px;
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s;
    display: flex;
    align-items: center;
    justify-content: center;
  }
  .btn-icon:hover { background: rgba(255,255,255,0.1); color: #f5f5f7; }
  .btn-icon.red:hover { background: rgba(255,69,58,0.2); color: #ff453a; }

  .error-box { 
    display: flex;
    align-items: center;
    gap: 10px;
    background: rgba(255,69,58,0.1);
    border: 1px solid rgba(255,69,58,0.3);
    color: #ff453a;
    padding: 12px;
    border-radius: 12px;
    font-size: 0.85rem;
  }

  .btn-icon-toolbar {
    background: transparent;
    border: none;
    color: #6e6e73;
    padding: 8px;
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.2s;
    display: flex;
    align-items: center;
    justify-content: center;
    text-decoration: none;
  }
  .btn-icon-toolbar:hover { background: rgba(255,255,255,0.1); color: #f5f5f7; }

  .lang-switcher {
    display: flex;
    gap: 4px;
    background: rgba(255,255,255,0.03);
    padding: 4px;
    border-radius: 8px;
    border: 1px solid rgba(255,255,255,0.06);
  }
  .lang-switcher button {
    background: transparent;
    border: none;
    color: #6e6e73;
    padding: 6px 10px;
    border-radius: 6px;
    cursor: pointer;
    font-size: 0.8rem;
    font-weight: 500;
    transition: all 0.2s;
  }
  .lang-switcher button:hover { background: rgba(255,255,255,0.05); color: #f5f5f7; }
  .lang-switcher button.active { background: rgba(41,151,255,0.2); color: #2997ff; }

  .modal-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0,0,0,0.7);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    backdrop-filter: blur(4px);
  }
  .modal-content {
    background: rgba(0,0,0,0.95);
    border: 1px solid rgba(255,255,255,0.06);
    border-radius: 20px;
    padding: 32px;
    max-width: 600px;
    max-height: 80vh;
    overflow-y: auto;
    backdrop-filter: blur(20px);
    scrollbar-width: thin;
    scrollbar-color: rgba(41,151,255,0.5) rgba(255,255,255,0.03);
  }
  .modal-content::-webkit-scrollbar {
    width: 8px;
  }
  .modal-content::-webkit-scrollbar-track {
    background: rgba(255,255,255,0.03);
    border-radius: 10px;
  }
  .modal-content::-webkit-scrollbar-thumb {
    background: linear-gradient(180deg, rgba(41,151,255,0.6), rgba(191,90,242,0.6));
    border-radius: 10px;
    border: 2px solid rgba(255,255,255,0.03);
  }
  .modal-content::-webkit-scrollbar-thumb:hover {
    background: linear-gradient(180deg, rgba(41,151,255,0.8), rgba(191,90,242,0.8));
  }
  .modal-header {
    font-size: 1.5rem;
    font-weight: 600;
    margin-bottom: 24px;
    color: #f5f5f7;
  }
  .tutorial-item {
    margin-bottom: 20px;
    padding-bottom: 20px;
    border-bottom: 1px solid rgba(255,255,255,0.06);
  }
  .tutorial-item:last-child { border-bottom: none; }
  .tutorial-item h4 {
    font-size: 1rem;
    font-weight: 600;
    color: #2997ff;
    margin-bottom: 8px;
  }
  .tutorial-item p {
    font-size: 0.9rem;
    color: #86868b;
    line-height: 1.5;
  }
  .modal-close {
    background: rgba(255,255,255,0.1);
    border: 1px solid rgba(255,255,255,0.2);
    color: #f5f5f7;
    padding: 12px 24px;
    border-radius: 12px;
    cursor: pointer;
    font-weight: 500;
    margin-top: 24px;
    width: 100%;
    transition: all 0.2s;
  }
  .modal-close:hover {
    background: rgba(255,255,255,0.15);
    border-color: rgba(255,255,255,0.3);
  }
</style>
