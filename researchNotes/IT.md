# Notes
## [Webcam-based gaze estimation for computer screen interaction](https://www.frontiersin.org/journals/robotics-and-ai/articles/10.3389/frobt.2024.1369566/full)
Lucas Falch & Katrin Solveig Lohan

Utilize OpenVINO (Open Visual Inference and Neural Network Optimization)'s gaze/face detection/head pose estimation/landmark estimation adas models
* Can be useful for our project so we don't have to build out the infrastructure for tracking the face
    * Can be more difficult as it is a new library

Gaze is given as a unit vector from the midpoint of the eyes and is projected onto the computer screen

Calibration is performed through looking at four points on the screen's corners to identify the bounds of the tracking
* Useful for reducing the mean absolute error of OpenVINO

Identify two types of motion in turning the head
* Left & Right
* Pitch & Yaw

## [Democratizing the Creation of Animatable Facial Avatars](https://arxiv.org/pdf/2401.16534)

Yilin Zhu, Dalton Omens, Haodi He & Ron Fedkiw

Front facing image can mislead between geometry and textures - they provide an example in Fig 1 and Fig 2 which losses details such as the nose and mouth and instead renders it as just a texture.
* Our current infrastructure has only beeen designed to work with a front facing point tracking model and that we will need to rework it in order to reduce the missing geometry.
* Their reconstruction involved a "few" pictures from different angles and distances.
    * This can be derived through a video (as it is simply a series of images)

Animation of reconstructed mesh was created using MetaHuman animation sytems
* Built by Epic Games
* Might be useful to reaseach further into for automated rigging of models and facial recreation
* Language infers that training data was created through the creation of the original training data through MetaHumans and could be useful for having some synthetic training data as it is difficult to gather real world ones

Useful to have textures without lighting baked in so that it appears correctly regardless of how the lighting is placed in a virtual enviroment.
* Not part of paper but I think it's possible to blend a series of images to average out the highlights and shadows induced through a light source

## ⭐ [A Systematic Literature Review: Facial Expression and Lip Movement Synchronization of an Audio Track](https://ieeexplore.ieee.org/stamp/stamp.jsp?tp=&arnumber=10536106)
Manal Hassan Alshahrani & Mashael Sulia Maashi

A review on lip synchronizing to an audio track. Can be referenced to find more sources and projects similiar to our own.
* Oldest Form - Lip Sync or translating sounds to corrosponding mounth shapes
* Automated phenome labelling and feature tracking to remap mouth movements to new audio tracks
* 3D wire frame models, replace with new 3d generated head with an adapted animation then restiched back in
* Using ANN, use neural network as classifier that focuses on the red color <- I can adapt our current infrasturcture to work with this.

Newer techniques include the understanding of the jawline which helps with creating animations that are believable.

Important to ensure that the model is inclusive of other cultures and accents. This is important to maintain authenticity and naturalness.

## [A markless 3D human motion data acquisition method based on the binocular stereo vision and lightweight open pose algorithm](https://www.sciencedirect.com/science/article/abs/pii/S0263224123014720)

Bo Sheng, Linfeng Chen, Jian Cheng, Yanxin Zhang, Zikai Hua, Jing Tao

Utilizies binocular cameras to identify depth and improve the accuracy of motion capturing
* Similiar to the multiple camera setups talked about in other papers but is more like the capture of 3D video or the bokeh simulation that we see on mobile phone setups.
* Is likely more accessible as most phones these days also have stereocopic camera setups

Preprocessing technique
* Grey scale and noise cancelling

Create a calibration board to help with understanding the scaling and depth
* What they used is a checkerboard

Utilized Open Pose Algorithm:
* Has three stages
    * Feature Extraction
        * I imagine that we can use color based as in right now
        * Contrast based
    * Initialization
        * They setup detection of skeleton points, we can setup detection of key points like jaw line
    * Refinement
        * Parameter count reduction, etc. not important for our purposes right now for brainstorming

Kinects were used as they mentioned "Kinect devices are often used for comparisons with gold standards, and their reliability and consistency have been experimentally validated"

## [HumanNorm: Learning Normal Diffusion Model for High-quality and Realistic 3D Human Generation](https://openaccess.thecvf.com/content/CVPR2024/html/Huang_HumanNorm_Learning_Normal_Diffusion_Model_for_High-quality_and_Realistic_3D_CVPR_2024_paper.html)

Xin Huang, Ruizhi Shao, Qi Zhang, Hongwen Zhang, Ying Feng, Yebin Liu & Qing Wang

Improving the detail of human generation through image processing by also learning a normal-adaped diffusion model. By generating fake 3D details as well as 3D textures it improves the realness of a face.

**Note**: Normal meaning the normal vector from a point on a mesh to induce an offset

I am thinking that they could be used in a way to improve details on a model taken on consumer hardware to look better. Maybe if we would like to use this in a way of a low bandwidth video communications format (aka. only capture movement data and recreate the person locally on the other end using a 3D model)

This is like a 3D model in games and 3dfx where normal maps are used to improve depth and shadows on would be flat models by including small offsets of vectors.

Steps they took:
* Generate human geometry
* Geometry based texture generation
* SDS Loss
    * Arguably the most important part as it ensures that features aren't exaggerated or generation model gaffs