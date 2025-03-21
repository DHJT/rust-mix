#include <stdio.h>

// 简单加法函数
int add(int a, int b) {
    return a + b;
}

// 结构体示例
typedef struct {
    int x;
    int y;
} Point;

// 结构体操作函数
void print_point(Point p) {
    printf("Point: (%d, %d)\n", p.x, p.y);
}