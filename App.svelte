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
  let obbWireframes = []; // Визуализация OBB боксов
  let obbClickMeshes = []; // Невидимые меши для клика по OBB

  // Ручной бокс
  let manualBox = null;
  let ghostBox = null;
  let creationPhase = 0;
  let startPoint = new THREE.Vector3();

  // Сидения
  let seats = [];
  let seatMode = false;
  let selectedSeatIndex = -1;

  // OBB визуализация (wireframe боксы)
  let obbHelpers = [];
  
  // OBB редактирование
  let obbEditMesh = null; // Меш для редактирования выбранного OBB
  
  // Группы костей модели (для очистки при загрузке новой)
  let boneGroups = [];

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
  let vehicleType = 'wheeled'; // 'wheeled' = машина (колёса отдельно), 'tracked' = танк (колёса объединены)
  let lastObbCount = 0; // количество OBB после последней генерации

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
      autoObb: 'Авто OBB',
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
      mode: 'Режим',
      vehicleType: 'Тип',
      wheeled: 'Машина',
      tracked: 'Танк',
      wasdMode: 'WASD режим · Мышь вращение · Ctrl быстрее',
      clickForSeat: 'Кликай по модели для сидения',
      clickToDraw: 'Кликай для рисования · Esc отмена',
      manualBoxHint: 'Ручной бокс · T Move · R Scale · Del удалить',
      obbSelected: 'OBB',
      obbEditHint: 'T Move · R Scale · Del удалить · Esc отмена',
      obbDeleteHint: 'Delete удалить · Esc отмена',
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
      autoObb: 'Auto OBB',
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
      mode: 'Mode',
      vehicleType: 'Type',
      wheeled: 'Wheeled',
      tracked: 'Tracked',
      wasdMode: 'WASD mode · Mouse rotate · Ctrl faster',
      clickForSeat: 'Click on model to add seat',
      clickToDraw: 'Click to draw · Esc cancel',
      manualBoxHint: 'Manual box · T Move · R Scale · Del delete',
      obbSelected: 'OBB',
      obbEditHint: 'T Move · R Scale · Del delete · Esc cancel',
      obbDeleteHint: 'Delete to remove · Esc cancel',
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
      autoObb: '自動OBB',
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
      mode: 'モード',
      vehicleType: 'タイプ',
      wheeled: '車両',
      tracked: '戦車',
      wasdMode: 'WASDモード · マウス回転 · Ctrl 高速',
      clickForSeat: 'モデルをクリックしてシートを追加',
      clickToDraw: 'クリックして描画 · Esc キャンセル',
      manualBoxHint: '手動ボックス · T 移動 · R スケール · Del 削除',
      obbSelected: 'OBB',
      obbEditHint: 'T 移動 · R スケール · Del 削除 · Esc キャンセル',
      obbDeleteHint: 'Delete 削除 · Esc キャンセル',
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
    // Сохраняем состояние OBB wireframes
    const obbState = obbWireframes.map(w => ({
      position: w.position.clone(),
      size: new THREE.Vector3().setFromMatrixScale(w.matrixWorld),
      boneName: w.userData.boneName
    }));
    
    // Получаем размеры из geometry
    const obbData = obbWireframes.map(w => {
      const box = new THREE.Box3().setFromObject(w);
      const size = new THREE.Vector3();
      box.getSize(size);
      return {
        position: w.position.clone(),
        size: size.clone(),
        boneName: w.userData.boneName
      };
    });
    
    const state = {
      selectedMeshes: new Set(selectedMeshes),
      hasManualBox: !!manualBox,
      manualBoxPos: manualBox ? manualBox.position.clone() : null,
      manualBoxScale: manualBox ? manualBox.scale.clone() : null,
      seats: seats.map(s => ({ 
        position: s.mesh.position.clone(), 
        boneName: s.boneName 
      })),
      obbData: obbData
    };
    history.push(state);
    if (history.length > MAX_HISTORY) history.shift();
  }

  function undo() {
    if (history.length === 0) return;
    const state = history.pop();
    
    // Снимаем выделение с OBB
    deselectOBB();
    
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
    
    // Восстанавливаем OBB
    if (state.obbData) {
      // Удаляем текущие OBB
      obbWireframes.forEach(w => {
        scene.remove(w);
        w.geometry.dispose();
        w.material.dispose();
      });
      obbWireframes = [];
      
      obbClickMeshes.forEach(m => {
        scene.remove(m);
        m.geometry.dispose();
        m.material.dispose();
      });
      obbClickMeshes = [];
      
      // Создаём OBB из сохранённого состояния
      state.obbData.forEach(obb => {
        const boxGeo = new THREE.BoxGeometry(obb.size.x, obb.size.y, obb.size.z);
        const edges = new THREE.EdgesGeometry(boxGeo);
        const lineMat = new THREE.LineBasicMaterial({ color: 0x00ff00, linewidth: 2 });
        const wireframe = new THREE.LineSegments(edges, lineMat);
        wireframe.position.copy(obb.position);
        wireframe.userData = { boneName: obb.boneName };
        scene.add(wireframe);
        obbWireframes.push(wireframe);
        
        const clickGeo = new THREE.BoxGeometry(obb.size.x, obb.size.y, obb.size.z);
        const clickMat = new THREE.MeshBasicMaterial({ visible: false });
        const clickMesh = new THREE.Mesh(clickGeo, clickMat);
        clickMesh.position.copy(obb.position);
        clickMesh.userData = { isOBB: true, boneName: obb.boneName, wireframe };
        scene.add(clickMesh);
        obbClickMeshes.push(clickMesh);
        
        boxGeo.dispose();
      });
    }
    
    regenerateOBBJson();
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
      // Обновляем OBB wireframe при редактировании
      if (obbEditMesh && selectedOBB && selectedOBB.wireframe) {
        updateOBBWireframe();
      }
    });
    transformControl.addEventListener('dragging-changed', (e) => { 
      controls.enabled = !e.value;
      // Сохраняем историю ПЕРЕД началом перетаскивания
      if (e.value && (manualBox || selectedSeatIndex >= 0 || obbEditMesh)) saveHistory();
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
      case 'delete': 
        if (selectedOBB) {
          deleteSelectedOBB();
        } else {
          deleteManualBox(); 
        }
        break;
      case 'escape': 
        cancelDrawing(); 
        selectedSeatIndex = -1;
        // Снимаем выделение с OBB
        deselectOBB();
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
    if (obbEditMesh) transformControl.setMode(mode);
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

  function getIntersect(e, includeSeatMeshes = false, includeOBB = false) {
    const rect = renderer.domElement.getBoundingClientRect();
    mouse.x = ((e.clientX - rect.left) / rect.width) * 2 - 1;
    mouse.y = -((e.clientY - rect.top) / rect.height) * 2 + 1;
    raycaster.setFromCamera(mouse, camera);
    const seatMeshes = includeSeatMeshes ? seats.map(s => s.mesh) : [];
    const obbMeshes = includeOBB ? obbClickMeshes : [];
    const objects = [...obbMeshes, ...seatMeshes, ...modelMeshes, invisiblePlane];
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

    // Проверяем клик по OBB (приоритет)
    const obbIntersect = getIntersect(e, false, true);
    if (obbIntersect && obbIntersect.object.userData && obbIntersect.object.userData.isOBB) {
      selectOBB(obbIntersect.object);
      return;
    }

    // Проверяем клик по сидению
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
      toggleMeshSelection(intersect.object);
    }
  }

  let selectedOBB = null;

  function selectOBB(clickMesh) {
    // Снимаем выделение с предыдущего OBB
    if (selectedOBB && selectedOBB.wireframe) {
      selectedOBB.wireframe.material.color.setHex(0x00ff00);
    }
    
    // Удаляем старый редактируемый меш
    if (obbEditMesh) {
      transformControl.detach();
      scene.remove(obbEditMesh);
      obbEditMesh.geometry.dispose();
      obbEditMesh.material.dispose();
      obbEditMesh = null;
    }
    
    selectedOBB = clickMesh.userData;
    
    // Подсвечиваем выбранный OBB желтым
    if (selectedOBB.wireframe) {
      selectedOBB.wireframe.material.color.setHex(0xffff00);
    }
    
    // Создаём редактируемый меш для OBB
    const wireframe = selectedOBB.wireframe;
    const geo = new THREE.BoxGeometry(1, 1, 1);
    const mat = new THREE.MeshStandardMaterial({ 
      color: 0xffaa00, 
      transparent: true, 
      opacity: 0.3,
      emissive: 0x332200
    });
    obbEditMesh = new THREE.Mesh(geo, mat);
    obbEditMesh.position.copy(wireframe.position);
    
    // Получаем размер из wireframe geometry
    const box = new THREE.Box3().setFromObject(wireframe);
    const size = new THREE.Vector3();
    box.getSize(size);
    obbEditMesh.scale.copy(size);
    
    scene.add(obbEditMesh);
    transformControl.attach(obbEditMesh);
    transformControl.setMode(editMode);
  }

  function deselectOBB() {
    if (selectedOBB && selectedOBB.wireframe) {
      selectedOBB.wireframe.material.color.setHex(0x00ff00);
    }
    if (obbEditMesh) {
      transformControl.detach();
      scene.remove(obbEditMesh);
      obbEditMesh.geometry.dispose();
      obbEditMesh.material.dispose();
      obbEditMesh = null;
    }
    selectedOBB = null;
  }

  function updateOBBWireframe() {
    if (!obbEditMesh || !selectedOBB || !selectedOBB.wireframe) return;
    
    const wireframe = selectedOBB.wireframe;
    const clickMesh = obbClickMeshes.find(m => m.userData.wireframe === wireframe);
    
    // Обновляем позицию wireframe
    wireframe.position.copy(obbEditMesh.position);
    
    // Обновляем размер wireframe (пересоздаём геометрию)
    const newSize = obbEditMesh.scale;
    wireframe.geometry.dispose();
    const boxGeo = new THREE.BoxGeometry(newSize.x, newSize.y, newSize.z);
    wireframe.geometry = new THREE.EdgesGeometry(boxGeo);
    boxGeo.dispose();
    
    // Обновляем click mesh
    if (clickMesh) {
      clickMesh.position.copy(obbEditMesh.position);
      clickMesh.geometry.dispose();
      clickMesh.geometry = new THREE.BoxGeometry(newSize.x, newSize.y, newSize.z);
    }
    
    // Обновляем JSON
    regenerateOBBJson();
  }

  function regenerateOBBJson() {
    // Пересобираем JSON из текущих wireframes
    const obbArray = [];
    
    obbWireframes.forEach((wireframe, index) => {
      const box = new THREE.Box3().setFromObject(wireframe);
      const size = new THREE.Vector3();
      const center = new THREE.Vector3();
      box.getSize(size);
      box.getCenter(center);
      
      obbArray.push({
        "Size": [toFloat(size.x / 2), toFloat(size.y / 2), toFloat(size.z / 2)],
        "Position": [toFloat(center.x * -1), toFloat(center.y), toFloat(center.z * -1)],
        "Part": wireframe.userData.boneName || 'Manual'
      });
    });
    
    // Генерация Seats
    const seatsArray = seats.map(s => {
      const pos = s.mesh.position;
      return {
        "HidePassenger": true,
        "BanHand": true,
        "Transform": s.boneName,
        "Position": [toFloat(pos.x * -1), toFloat(pos.y), toFloat(pos.z * -1)]
      };
    });

    if (obbArray.length === 0 && seatsArray.length === 0) {
      outputText = '';
      return;
    }

    let parts = [];
    if (obbArray.length > 0) {
      const obbItems = obbArray.map(o => 
        `{\n  "Size": [${o.Size.join(', ')}],\n  "Position": [${o.Position.join(', ')}],\n  "Part": "${o.Part}"\n}`
      );
      parts.push(`"OBB": [\n${obbItems.join(',\n')}\n]`);
    }
    if (seatsArray.length > 0) {
      const seatsItems = seatsArray.map(s => 
        `{\n  "HidePassenger": ${s.HidePassenger},\n  "BanHand": ${s.BanHand},\n  "Transform": "${s.Transform}",\n  "Position": [${s.Position.join(', ')}]\n}`
      );
      parts.push(`"Seats": [\n${seatsItems.join(',\n')}\n]`);
    }
    outputText = parts.join(',\n');
  }

  function deleteSelectedOBB() {
    if (!selectedOBB) return;
    
    // Удаляем wireframe и click mesh
    const wireframe = selectedOBB.wireframe;
    const clickMeshIndex = obbClickMeshes.findIndex(m => m.userData.wireframe === wireframe);
    const wireframeIndex = obbWireframes.indexOf(wireframe);
    
    if (wireframeIndex !== -1) {
      scene.remove(wireframe);
      wireframe.geometry.dispose();
      wireframe.material.dispose();
      obbWireframes.splice(wireframeIndex, 1);
    }
    
    if (clickMeshIndex !== -1) {
      const clickMesh = obbClickMeshes[clickMeshIndex];
      scene.remove(clickMesh);
      clickMesh.geometry.dispose();
      clickMesh.material.dispose();
      obbClickMeshes.splice(clickMeshIndex, 1);
    }
    
    // Удаляем edit mesh
    deselectOBB();
    
    // Обновляем JSON
    regenerateOBBJson();
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

    // Удаляем старые OBB wireframes и click meshes
    obbWireframes.forEach(w => {
      scene.remove(w);
      w.geometry.dispose();
      w.material.dispose();
    });
    obbWireframes = [];
    
    obbClickMeshes.forEach(m => {
      scene.remove(m);
      m.geometry.dispose();
      m.material.dispose();
    });
    obbClickMeshes = [];

    // Группируем выделенные меши по костям
    const selectedBoneGroups = new Map();
    
    // Паттерны для игнорирования (только траки гусениц)
    const ignorePatterns = [
      /^track/i, /^GUS\d/i, /^gus$/i,  // траки гусениц
    ];
    
    // Паттерны для объединения в группы (корпус, кабина и т.д.)
    const mergeGroups = [
      // Корпус/база/кабина/corpus - всё объединяем в body
      { pattern: /^(base|body|cubedfdf|OPORY|CABIN|CABINA|door|hatch|wiper|Stering|whel$|corpus$|frame)/i, name: 'body' },
      // Верх башни (bone, turret) - отдельно вращается
      { pattern: /^(bone$|turret$)/i, name: 'turret' },
      // Пушка/ствол - отдельно от башни
      { pattern: /^(barrel|Gun)/i, name: 'barrel' },
      // Радар/сенсоры - отдельно
      { pattern: /^(RADAR|radar)$/i, name: 'radar' },
      // Двигатель
      { pattern: /^(engine|Wheel$)/i, name: 'engine' },
      // group1, group2... → body тоже
      { pattern: /^group\d*$/i, name: 'body' },
      // Броня/защита
      { pattern: /^(SIDE|DZ|RELICT|homework|ATGM)/i, name: 'armor' },
    ];
    
    // Функция определения группы для объединения
    function getMergeGroup(boneName, meshes) {
      // Для танков (tracked) объединяем колёса по позиции X
      if (vehicleType === 'tracked') {
        // Проверяем, похоже ли на колесо по имени
        const isWheel = /^(wheel|whell|whl|koleso)/i.test(boneName);
        if (isWheel && meshes && meshes.length > 0) {
          // Определяем сторону по позиции X первого меша
          const worldPos = new THREE.Vector3();
          meshes[0].getWorldPosition(worldPos);
          if (worldPos.x > 0.1) {
            return 'WheelLeft'; // В Three.js X > 0 это левая сторона (инверсия)
          } else if (worldPos.x < -0.1) {
            return 'WheelRight';
          }
        }
      }
      
      for (const group of mergeGroups) {
        if (group.pattern.test(boneName)) {
          return group.name;
        }
      }
      return boneName; // если не подходит ни под одну группу - оставляем как есть
    }
    
    // Функция проверки "антенноподобности" кости
    function isAntennaLike(meshes) {
      const box = new THREE.Box3();
      meshes.forEach(mesh => {
        const meshBox = new THREE.Box3().setFromObject(mesh);
        box.union(meshBox);
      });
      const size = new THREE.Vector3();
      box.getSize(size);
      
      const sizes = [size.x, size.y, size.z].sort((a, b) => b - a);
      const aspectRatio = sizes[0] / sizes[2]; // самая длинная / самая короткая
      // Антенна: очень вытянутая (>6:1) и тонкая (<0.3)
      return aspectRatio > 6 && sizes[2] < 0.3;
    }
    
    // Сначала группируем меши по костям
    const boneToMeshes = new Map();
    selectedMeshes.forEach(meshId => {
      const mesh = modelMeshes.find(m => m.uuid === meshId);
      if (!mesh || !mesh.userData) return;
      
      let boneName = mesh.userData.boneName;
      
      // Проверяем игнорируемые паттерны
      if (ignorePatterns.some(p => p.test(boneName))) {
        return;
      }
      
      if (!boneToMeshes.has(boneName)) {
        boneToMeshes.set(boneName, []);
      }
      boneToMeshes.get(boneName).push(mesh);
    });
    
    // Теперь группируем кости, но антенноподобные оставляем отдельно (они отфильтруются позже)
    boneToMeshes.forEach((meshes, boneName) => {
      // Проверяем, антенна ли это
      if (isAntennaLike(meshes)) {
        // Антенны не добавляем в группы - они отфильтруются по размеру позже
        return;
      }
      
      // Определяем группу для объединения (передаём меши для определения позиции колёс)
      const groupName = getMergeGroup(boneName, meshes);
      
      if (!selectedBoneGroups.has(groupName)) {
        selectedBoneGroups.set(groupName, []);
      }
      // Добавляем все меши этой кости в группу
      meshes.forEach(mesh => selectedBoneGroups.get(groupName).push(mesh));
    });

    // Функция проверки "антенноподобности" отдельного меша
    function isMeshAntennaLike(mesh) {
      const box = new THREE.Box3().setFromObject(mesh);
      const size = new THREE.Vector3();
      box.getSize(size);
      
      const sizes = [size.x, size.y, size.z].sort((a, b) => b - a);
      const aspectRatio = sizes[0] / sizes[2];
      // Антенна: очень вытянутая (>8:1) и тонкая (<0.15)
      return aspectRatio > 8 && sizes[2] < 0.15;
    }

    // Сначала собираем все OBB с их Box3
    const tempObbList = [];
    
    selectedBoneGroups.forEach((meshes, groupName) => {
      const combinedBox = new THREE.Box3();
      
      meshes.forEach(mesh => {
        // Пропускаем антенноподобные кубы
        if (isMeshAntennaLike(mesh)) {
          return;
        }
        const meshBox = new THREE.Box3().setFromObject(mesh);
        combinedBox.union(meshBox);
      });
      
      // Если после фильтрации ничего не осталось - пропускаем
      if (combinedBox.isEmpty()) {
        return;
      }

      const worldSize = new THREE.Vector3();
      const worldCenter = new THREE.Vector3();
      combinedBox.getSize(worldSize);
      combinedBox.getCenter(worldCenter);

      // Фильтр по размеру - игнорируем слишком маленькие/тонкие OBB
      const minThickness = 0.1; // минимальная толщина
      
      // Пропускаем если слишком тонкий (любой размер < minThickness)
      if (worldSize.x < minThickness || worldSize.y < minThickness || worldSize.z < minThickness) {
        return;
      }
      
      // Фильтр антенн - если одна сторона намного больше других (соотношение > 10:1)
      const sizes = [worldSize.x, worldSize.y, worldSize.z].sort((a, b) => b - a);
      const aspectRatio = sizes[0] / sizes[2]; // самая длинная / самая короткая
      if (aspectRatio > 10 && sizes[2] < 0.15) {
        return; // это антенна или провод
      }

      const size = [
        toFloat(worldSize.x / 2),
        toFloat(worldSize.y / 2),
        toFloat(worldSize.z / 2)
      ];

      const position = [
        toFloat(worldCenter.x * -1),
        toFloat(worldCenter.y),
        toFloat(worldCenter.z * -1)
      ];

      // Сохраняем во временный список с Box3 для проверки вложенности
      tempObbList.push({
        size,
        position,
        part: groupName,
        box3: combinedBox.clone(),
        worldSize: worldSize.clone(),
        worldCenter: worldCenter.clone()
      });
    });

    // Фильтруем OBB которые полностью внутри других
    const filteredObbList = tempObbList.filter((obb, index) => {
      // Проверяем, не находится ли этот OBB полностью внутри другого большего OBB
      for (let i = 0; i < tempObbList.length; i++) {
        if (i === index) continue;
        const other = tempObbList[i];
        // Если other полностью содержит obb
        if (other.box3.containsBox(obb.box3)) {
          // Проверяем что other больше (чтобы не удалить оба)
          const obbVolume = obb.worldSize.x * obb.worldSize.y * obb.worldSize.z;
          const otherVolume = other.worldSize.x * other.worldSize.y * other.worldSize.z;
          if (otherVolume > obbVolume * 1.5) { // other должен быть минимум в 1.5 раза больше
            return false; // удаляем этот OBB
          }
        }
      }
      return true;
    });

    // Создаём OBB и wireframes для отфильтрованного списка
    filteredObbList.forEach(obb => {
      obbArray.push({ "Size": obb.size, "Position": obb.position, "Part": obb.part });

      // Создаем wireframe визуализацию OBB (зеленые линии)
      const boxGeo = new THREE.BoxGeometry(obb.worldSize.x, obb.worldSize.y, obb.worldSize.z);
      const edges = new THREE.EdgesGeometry(boxGeo);
      const lineMat = new THREE.LineBasicMaterial({ color: 0x00ff00, linewidth: 2 });
      const wireframe = new THREE.LineSegments(edges, lineMat);
      wireframe.position.copy(obb.worldCenter);
      wireframe.userData = { obbIndex: obbArray.length - 1, boneName: obb.part };
      scene.add(wireframe);
      obbWireframes.push(wireframe);
      
      // Невидимый mesh для клика
      const clickGeo = new THREE.BoxGeometry(obb.worldSize.x, obb.worldSize.y, obb.worldSize.z);
      const clickMat = new THREE.MeshBasicMaterial({ visible: false });
      const clickMesh = new THREE.Mesh(clickGeo, clickMat);
      clickMesh.position.copy(obb.worldCenter);
      clickMesh.userData = { isOBB: true, obbIndex: obbArray.length - 1, boneName: obb.part, wireframe };
      scene.add(clickMesh);
      obbClickMeshes.push(clickMesh);
      
      boxGeo.dispose();
    });

    if (manualBox) {
      const pos = manualBox.position;
      const scale = manualBox.scale;

      obbArray.push({
        "Size": [toFloat(scale.x / 2), toFloat(scale.y / 2), toFloat(scale.z / 2)],
        "Position": [toFloat(pos.x * -1), toFloat(pos.y), toFloat(pos.z * -1)],
        "Part": "Manual"
      });

      // Wireframe для manual box
      const boxGeo = new THREE.BoxGeometry(scale.x, scale.y, scale.z);
      const edges = new THREE.EdgesGeometry(boxGeo);
      const lineMat = new THREE.LineBasicMaterial({ color: 0x00ff00, linewidth: 2 });
      const wireframe = new THREE.LineSegments(edges, lineMat);
      wireframe.position.copy(pos);
      scene.add(wireframe);
      obbWireframes.push(wireframe);
      boxGeo.dispose();
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

  function generateAutoOBB() {
    if (modelMeshes.length === 0) return;
    
    saveHistory();
    
    // Добавляем все кубы в selectedMeshes БЕЗ перекраски (для производительности)
    modelMeshes.forEach(mesh => {
      selectedMeshes.add(mesh.uuid);
    });
    
    // Генерируем OBB
    updateJSON();
    
    // Сохраняем количество OBB и очищаем выделение (чтобы не показывать 618 кубов)
    lastObbCount = obbWireframes.length;
    selectedMeshes.clear();
    selectedMeshes = selectedMeshes;
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
    // Удаляем старые группы костей из сцены
    boneGroups.forEach(g => {
      scene.remove(g);
    });
    boneGroups = [];
    
    modelMeshes.forEach(m => { 
      m.geometry.dispose(); 
      m.material.dispose(); 
    });
    modelMeshes = [];
    selectedMeshes.clear();
    history = [];
    if (manualBox) deleteManualBox(false);
    
    // Очищаем сидения
    seats.forEach(s => {
      scene.remove(s.mesh);
      s.mesh.geometry.dispose();
      s.mesh.material.dispose();
    });
    seats = [];
    selectedSeatIndex = -1;
    
    // Очищаем OBB wireframes и click meshes
    obbWireframes.forEach(w => {
      scene.remove(w);
      w.geometry.dispose();
      w.material.dispose();
    });
    obbWireframes = [];
    
    obbClickMeshes.forEach(m => {
      scene.remove(m);
      m.geometry.dispose();
      m.material.dispose();
    });
    obbClickMeshes = [];
    selectedOBB = null;
    
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
        boneGroups.push(node.group); // Сохраняем корневые группы для очистки
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
          ⬜️ {t.manualBoxHint}
        {:else if selectedOBB}
          <span style="color:#ffff00">📦 {t.obbSelected}: {selectedOBB.boneName}</span> | {t.obbEditHint}
        {:else if obbWireframes.length > 0 && selectedMeshes.size === 0}
          📦 OBB: <b style="color:#00ff00">{obbWireframes.length}</b> | {t.clickSelect}
        {:else if modelMeshes.length > 0}
          🖱️ {t.clickSelect} | {t.mode}: <b style="color:#ff9800">{t.cube}</b> | {t.selected}: <b style="color:#00ff00">{selectedMeshes.size}</b>
        {:else}
          {t.loadFile}
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
          <button class="mode-btn" on:click={generateAutoOBB}>
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="3" y="3" width="18" height="18" rx="2"/><path d="M3 9h18"/><path d="M9 21V9"/></svg>
            {t.autoObb}
          </button>
        </div>
        <div class="vehicle-type-toggle">
          <button class="vehicle-btn" class:active={vehicleType === 'wheeled'} on:click={() => vehicleType = 'wheeled'}>
            <svg class="vehicle-icon" width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <rect x="3" y="8" width="18" height="8" rx="2"/>
              <circle cx="7" cy="16" r="2.5"/>
              <circle cx="17" cy="16" r="2.5"/>
              <path d="M5 8V6a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v2"/>
            </svg>
            <span class="vehicle-label">{t.wheeled}</span>
          </button>
          <button class="vehicle-btn" class:active={vehicleType === 'tracked'} on:click={() => vehicleType = 'tracked'}>
            <svg class="vehicle-icon" width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <!-- Гусеницы -->
              <rect x="2" y="12" width="20" height="6" rx="3"/>
              <!-- Корпус -->
              <path d="M4 12V9a1 1 0 0 1 1-1h14a1 1 0 0 1 1 1v3"/>
              <!-- Башня -->
              <rect x="8" y="5" width="8" height="3" rx="1"/>
              <!-- Пушка -->
              <path d="M16 6.5h5"/>
            </svg>
            <span class="vehicle-label">{t.tracked}</span>
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
  .mode-btn.small {
    padding: 6px 10px;
    font-size: 0.8rem;
  }
  .mode-btn:hover { background: rgba(255,255,255,0.05); color: #f5f5f7; }
  .mode-btn.active { background: rgba(255,255,255,0.1); color: #fff; }
  .mode-btn svg { width: 16px; height: 16px; }

  /* Vehicle type toggle */
  .vehicle-type-toggle {
    display: flex;
    gap: 8px;
    margin-top: 10px;
  }
  .vehicle-btn {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
    padding: 10px 8px;
    border: 1px solid rgba(255,255,255,0.1);
    background: rgba(255,255,255,0.03);
    border-radius: 12px;
    cursor: pointer;
    transition: all 0.2s;
  }
  .vehicle-btn:hover {
    background: rgba(255,255,255,0.06);
    border-color: rgba(255,255,255,0.15);
  }
  .vehicle-btn.active {
    background: rgba(46, 204, 113, 0.15);
    border-color: rgba(46, 204, 113, 0.4);
  }
  .vehicle-icon {
    width: 28px;
    height: 28px;
    color: #86868b;
    transition: color 0.2s;
  }
  .vehicle-btn.active .vehicle-icon {
    color: #2ecc71;
  }
  .vehicle-label {
    font-size: 0.75rem;
    color: #86868b;
    font-weight: 500;
  }
  .vehicle-btn.active .vehicle-label {
    color: #2ecc71;
  }

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
