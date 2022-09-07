#include "constraints.h"

#define MIN_MASTER_AREA_FACTOR 0.05
#define MAX_MASTER_AREA_FACTOR 0.95
#define MIN_SNAP_DISTANCE 1
#define MAX_SNAP_DISTANCE 10000

float constraints_default_master_area_factor(const float default_master_area_factor)
{
	return constraints_master_area_factor(default_master_area_factor);
}

float constraints_master_area_factor(const float master_area_factor)
{
	if (master_area_factor < MIN_MASTER_AREA_FACTOR) return MIN_MASTER_AREA_FACTOR;
	if (master_area_factor > MAX_MASTER_AREA_FACTOR) return MAX_MASTER_AREA_FACTOR;
	return master_area_factor;
}

unsigned int constraints_snap_distance(const unsigned int snap_distance)
{
	if (snap_distance < MIN_SNAP_DISTANCE) return MIN_SNAP_DISTANCE;
	if (snap_distance > MAX_SNAP_DISTANCE) return MAX_SNAP_DISTANCE;
	return snap_distance;
}
