#include "world_transform.h"

void WorldTrans::SetScale(float scale){
	m_scale=scale;
}

void WorldTrans::SetRotation(float x, float y, float z){
	m_rotation.x=x;
	m_rotation.y=y;
	m_rotation.z=z;
}

void WorldTrans::SetPosition(float x, float y, float z){
	m_pos.x=x;
	m_pos.y=y;
	m_pos.z=z;
}

void WorldTrans::Rotate(float x, float y, float z){
	m_rotation.x+=x;
	m_rotation.y+=y;
	m_rotation.z+=z;
}

Matrix4f WorldTrans::GetMatrix(){
	Matrix4f Scale, Rotation, Translation;
	Scale.InitScaleTransform(m_scale, m_scale, m_scale);
	Rotation.InitRotateTransform(m_rotation.x, m_rotation.y, m_rotation.z);
	Translation.InitTranslationTransform(m_pos.x, m_pos.y, m_pos.z);

	return Translation * Rotation * Scale;
}

