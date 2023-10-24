// kpns.c
#include <stdint.h>
#include <stdlib.h>

uint64_t next_id = 0;

void free_heap(void *ptr) {
    free(ptr);
}

typedef struct
{
    float x;
    float y;
    float yaw;
    uint64_t id;
} VehiclePose;

VehiclePose create(float x, float y, float yaw)
{
    VehiclePose pose = {
        .x = x,
        .y = y,
        .yaw = yaw,
        .id = next_id,
    };
    next_id++;
    return pose;
}

void translate(VehiclePose *vehicles, uint64_t n, float dx, float dy)
{
    for (uint64_t i = 0; i < n; i++)
    {
        vehicles[i].x += dx;
        vehicles[i].y += dy;
    }
}

VehiclePose *create_heap()
{
    VehiclePose *pose = malloc(sizeof(VehiclePose));
    if (pose == NULL)
    {
        return NULL;
    }

    pose->id = next_id;
    next_id++;
    return pose;
}