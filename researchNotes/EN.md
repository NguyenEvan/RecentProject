# Displaced Dynamic Expression Regression for Real-time Facial Tracking and Animation 
## by Chen Cao, Qiming Hou, and Kun Zhou 


### Abstract
- introduces a fully automatic approach to real-time facial tracking and animation with a single video camera 
- does not require individual calibration for each user
- uses a generic regressor trained from public image datasets to infer accurate 2D facial landmarks and 3D facial shape from 2D video frames
- inferred data is used to adapt the camera matrix and the user's identity to better match their facial expressions
- regression and adaptation steps alternate in a feedback loop, allowing the system to converge quickly and accurately

### Introduction
- proliferation of commodity RGBD cameras (Microsoft's Kinect) has enhanced performance-driven facial animation,
    - this has become accessible for consumer-level applications
- video-based facial tracking remains difficult due to the wider availability of video cameras on PCs and mobile devices compared to RGBD cameras

### Methodology
- proposed approach eliminates the need for user-specific calibration by learning a generic regressor from large public datasets
- regressor is capable of inferring 2D landmarks and 3D facial shapes from video frames for any user and camera setup
- inferred landmarks are used to adjust the camera matrix and user identity iteratively, creating a feedback loop that improves tracking accuracy over time
- better robustness and accuracy comparable to techniques that require individual calibration, running at an average speed of 28 frames per second

### Results and Comparison
- paper compares their approach to two state-of-the-art techniques
    - a user-specific regression algorithm 
    - a 3D Constrained Local Model (CLM) approach. 
- paper's method shows comparable tracking accuracy to the user-specific method and outperforms the 3D CLM approach, especially in scenarios with large facial rotations and varying lighting conditions
- validation using depth data from a Kinect camera demonstrates that their method closely approximates the ground truth depth values without using depth information directly

### Conclusion
- calibration-free approach is a significant advancement for real-time facial tracking and animation, offering a robust, accurate, and efficient solution suitable for wide deployment in consumer applications
- Displaced Dynamic Expression (DDE) model combines the advantages of 3D dynamic expression models and 2D landmarks, enhancing the overall tracking performance

# A Real-Time Face Tracking and Animation System

### Abstract
- introduces a system that combines real-time infra-red (IR) based active facial feature tracking with real-time facial expression generation
- extracts 22 feature points, head pose orientation, and eye open-close status from video input. These features are then used to animate a 3D face model.
- potential applications in human-computer interaction, model-based video conferencing, and avatar communication.

### Introduction
- system addresses contemporary challenges
    - real-time feature tracking under varying imaging conditions (lighting, pose changes, and occlusions) 
    - realistic face modeling using limited feature parameters.

### System Overview
The system consists of three main modules:
- IR-Based Active Face Tracking: Utilizes an IR sensitive camera and LEDs to detect eye pupils and track facial features under various lighting conditions.
- Feature Parameter Adaptation: Converts 2D motion data to 3D animation data for an individualized 3D model.
- Dynamic Inference Algorithm: Generates animations for non-feature vertices using a coarse-to-fine strategy.
### 1. Active Face Tracking
- IR-Based Eye Tracking: Detects eye pupils using IR illumination to differentiate between bright and dark pupil images.
- Initial Feature Detection Using Gabor Wavelet: Identifies 22 facial features around the eyes and mouth using Gabor wavelet representation.
- Feature Tracking by Kalman Filter: Uses Kalman filtering to predict and track feature locations, combining head and pupil motions.
- Face Pose Tracking: Tracks the 3D face pose using detected eyes and anthropometric statistics, synchronizing face detection and pose estimation
### 2. and 3. 3D Animation Data Generation
- Transfer from Source Motion Parameters (MPs) to Target Animation Parameters (APs): Adapts 2D motion parameters from the tracking system to 3D animation parameters based on a "depth pattern" and head orientation vector
- Scaling and Normalization: Adapts 3D animation parameters to the individual 3D avatar, transforming the APs to the front view for deformation of non-feature vertices
- Region-Based Dynamic Inference Algorithm: Generates animations for non-feature vertices

# Imitator: Personalized Speech-driven 3D Facial Animation
### Definitions
- Bilabial: speech sounds produced by bringing both lips together
- Viseme: represents the shape and movement of the mouth, lips, and other facial features when pronouncing a particular sound
### Abstract
- "Imitator," a novel method for personalized speech-driven 3D facial animation
- Imitator considers identity-specific speaking styles and facial idiosyncrasies to produce more realistic and accurate lip movements
- uses a style-agnostic transformer trained on a large facial expression dataset to generate person-specific motion sequences from audio input
- optimized for identity-specific speaking styles using a short reference video and introduces a novel loss function for bilabial consonants to ensure accurate lip closures

### Introduction
3D digital humans aim to replicate real human appearance and motion for applications such as telepresence, gaming, and virtual reality

### Methodology/Model Architecture
- Audio Encoder: Utilizes Wav2Vec 2.0 to encode audio inputs into a meaningful latent representation.
- Auto-regressive Viseme Decoder: Generates style-agnostic viseme features from audio features using a transformer architecture.
- Motion Decoder: Maps viseme features to identity-specific facial animations using a style embedding layer and a motion synthesis block.
### Training:
- trained on the VOCA dataset, includes high-quality 3D face reconstructions and audio data
- a short reference video of the target subject is used to optimize the style embedding and refine the motion basis

### Results
Imitator outperformed state-of-the-art methods like VOCA and FaceFormer in Lip-Sync accuracy and overall expressiveness 
- achieved better performance on metrics like Lip-Sync, Lip-max, and overall facial animation accuracy compared to baselines
- generated animations were more expressive and accurate in lip movements, especially for bilabial consonants.
- robustness against noisy audio inputs



