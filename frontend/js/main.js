import { BronzeDrum3D } from './bronze_drum_3d.js';
import { AcousticPanel } from './acoustic_panel.js';
import { ExperiencePanel } from './experience_panel.js?v=3';

const drum3D = new BronzeDrum3D('three-canvas');
const panel = new AcousticPanel(drum3D);
const experience = new ExperiencePanel();

panel.init();
experience.init();
