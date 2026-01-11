# 🧊 OBB Lab

**Professional OBB (Oriented Bounding Box) Generator for Minecraft modding**

[![Live Demo](https://img.shields.io/badge/🚀_Live_Demo-OBB_Lab-2997ff?style=for-the-badge)](https://denys-shatin.github.io/OBB-Lab/)
[![License](https://img.shields.io/badge/License-MIT-green?style=for-the-badge)](LICENSE)
[![Svelte](https://img.shields.io/badge/Svelte-4A4A55?style=for-the-badge&logo=svelte&logoColor=FF3E00)](https://svelte.dev/)
[![Three.js](https://img.shields.io/badge/Three.js-000000?style=for-the-badge&logo=three.js&logoColor=white)](https://threejs.org/)

## ✨ Features

### 🎯 **Smart Selection System**
- **Cube Mode**: Select individual cubes with precision
- **Bone Mode**: Select entire bone structures at once
- Visual feedback with color-coded selection

### 🎨 **3D Visualization**
- Real-time 3D model rendering with Three.js
- Professional lighting and shadows
- Smooth camera controls with WASD navigation
- Apple-style UI design with glass morphism

### 🛠️ **Advanced Tools**
- **Manual Box Drawing**: Create custom hitboxes by clicking two points
- **Seat Placement**: Add player seating positions with visual markers
- **Transform Controls**: Move and scale objects with T/R hotkeys
- **Undo System**: Ctrl+Z support with 50-step history

### 🌍 **Multi-Language Support**
- 🇷🇺 Russian (auto-detect for RU, UA, BY, KZ)
- 🇺🇸 English (default for most countries)
- 🇯🇵 Japanese (auto-detect for Japan)
- Smart language detection based on browser locale

### 📱 **Modern Interface**
- Responsive design that works on all devices
- Dark theme with Apple-style aesthetics
- Interactive tutorial system
- Real-time JSON output with copy functionality

## 🚀 Quick Start

### Online Usage (Recommended)
Visit **[OBB Lab](https://denys-shatin.github.io/OBB-Lab/)** - no installation required!

### Local Development
```bash
# Clone the repository
git clone https://github.com/denys-shatin/OBB-Lab.git
cd OBB-Lab

# Install dependencies
npm install

# Start development server
npm run dev
```

## 📖 How to Use

1. **📁 Load Model**: Click "Upload" and select your `.geo.json` Minecraft Bedrock model file
2. **🎯 Select Elements**: 
   - Use **Cube mode** to select individual cubes
   - Use **Bone mode** to select entire bone structures
3. **✏️ Draw Custom Boxes**: Click "Draw Box" and click two points to create custom hitboxes
4. **💺 Add Seats**: Enable seat mode and click on the model to place player seating positions
5. **🎮 Navigate**: 
   - **Selection Mode**: Standard orbit controls (LMB rotate, MMB zoom, RMB pan)
   - **WASD Mode**: Game-like flying controls (WASD + Space/Shift, Ctrl for speed boost)
6. **📋 Export**: Copy the generated JSON configuration for your Minecraft addon

## 🎮 Controls

### Selection Mode
- **Left Click**: Rotate camera
- **Middle Click**: Zoom
- **Right Click**: Pan camera
- **Click on Model**: Select cubes/bones

### WASD Mode
- **W/A/S/D** (or **Ц/Ф/Ы/В**): Move camera
- **Space**: Move up
- **Shift**: Move down  
- **Ctrl**: Move faster
- **Mouse**: Rotate view

### Universal
- **Ctrl+Z**: Undo last action
- **T**: Switch to Move tool
- **R**: Switch to Scale tool
- **Delete**: Remove selected manual box
- **Escape**: Cancel current operation

## 🔧 Technical Details

### Built With
- **[Svelte](https://svelte.dev/)** - Reactive UI framework
- **[Three.js](https://threejs.org/)** - 3D graphics library
- **[Vite](https://vitejs.dev/)** - Fast build tool

### Browser Support
- Chrome/Edge 90+
- Firefox 88+
- Safari 14+

### File Format
Supports Minecraft Bedrock `.geo.json` model files with:
- Bone hierarchies
- Cube geometries  
- Rotation and pivot data
- Texture UV mapping

## 📊 Output Format

The tool generates JSON configuration for Minecraft Bedrock addons:

```json
{
  "OBB": [
    {
      "Size": [1.0, 1.0, 1.0],
      "Position": [0.0, 0.0, 0.0], 
      "Part": "body"
    }
  ],
  "Seats": [
    {
      "HidePassenger": true,
      "BanHand": true,
      "Transform": "seat_bone",
      "Position": [0.0, 1.2, 0.0]
    }
  ]
}
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the project
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Minecraft Bedrock Edition model format
- Three.js community for excellent 3D tools
- Svelte team for the amazing framework
- Apple for design inspiration

---

<div align="center">

**[🚀 Try OBB Lab Now](https://denys-shatin.github.io/OBB-Lab/)**

Made with ❤️ for the Minecraft modding community

</div>
