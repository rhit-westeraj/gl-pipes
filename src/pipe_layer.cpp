/**
 * @file node_struct.cpp
 * @author Alex "FaceFTW" Westerman
 * @brief
 * @version 0.1
 * @date 2022-06-27
 *
 * @copyright Copyright (c) 2022. Work is based on original work from Microsoft Corp (c) 1994
 *
 */
#pragma once
#include "include/pipe_layer.h"
#include "include/utils.h"
#include <cstddef>

using namespace GlPipes;

/******CONSTRUCTORS/DESTRUCTORS*******/
PipeLayer::PipeLayer() {
	node_struct_size = new Point(10, 10, 10);
	pipes = new Pipe*[6];
	node_struct = new PipePart***[node_struct_size->x];

	for(int i = 0; i < node_struct_size->x; i++) {
		node_struct[i] = new PipePart**[node_struct_size->y];
		for(int j = 0; j < node_struct_size->y; j++) {
			node_struct[i][j] = new PipePart*[node_struct_size->z];
		}
	}
}

PipeLayer::PipeLayer(Point* node_size, int numPipes) {
	pipes = new Pipe*[numPipes];
	node_struct = new PipePart***[node_size->x];
	node_struct_size = node_size;

	for(int i = 0; i < node_size->x; i++) {
		node_struct[i] = new PipePart**[node_size->y];
		for(int j = 0; j < node_size->y; j++) { node_struct[i][j] = new PipePart*[node_size->z]; }
	}
}

PipeLayer::~PipeLayer() {
	//PERF Very inelegant, prob a better way of doing this, but still;
	for(int i = 0; i < node_struct_size->x; i++) {
		for(int j = 0; j < node_struct_size->y; j++) { delete[] node_struct[i][j]; }
		delete[] node_struct[i];
	}
	delete[] node_struct;
	delete[] pipes;

	delete node_struct_size;
}

/******OVERLOADED OPERATORS*******/
Pipe*& PipeLayer::operator()(int pipeIdx) { return pipes[pipeIdx]; }

PipePart*& PipeLayer::operator[](Point* pos) { return node_struct[pos->x][pos->y][pos->z]; }

void PipeLayer::generatePipe(int pipeIdx) {
	Point* startPos = findRandomEmptyNode();
	pipes[pipeIdx] = new Pipe(startPos);

	//Choose a random starting direction that is empty

	//Add a Starting Pipe Node

	//Choose a random number of iterations (minimum 5 to maximum 10 for now)
	int numIter = iRand2(5, 10);

	for(int i = 0; i < numIter; i++) {
		//Determine how many nodes are available in that direction

		//Choose a random number n, 1 < n < available nodes

		//For n-1 available nodes, create straight pipe segs in dir
		//Logically, if only one node is available, this will not run

		//For the nth node (is not a loop)
		//choose a random empty direction
		//Add a pipe joint that turns to that direction
	}

	//Pop the last node, replace it with a Pipe Ending Node

	//Profit
}

bool PipeLayer::isEmpty(Point* pos) { return node_struct[pos->x][pos->y][pos->z] != NULL; }

Point* PipeLayer::getNextNodePos(Point* curPos, Direction dir) {
	switch(dir) {
		case DIR_X_PLUS:
			if((curPos->x) + 1 < this->size(AXIS_X)) return new Point(curPos, DIR_X_PLUS);
			break;
		case DIR_X_MINUS:
			if((curPos->x) - 1 < 0) return new Point(curPos, DIR_X_MINUS);
			break;
		case DIR_Y_PLUS:
			if((curPos->y) + 1 < this->size(AXIS_Y)) return new Point(curPos, DIR_Y_PLUS);
			break;
		case DIR_Y_MINUS:
			if((curPos->y) - 1 < 0) return new Point(curPos, DIR_Y_MINUS);
			break;
		case DIR_Z_PLUS:
			if((curPos->z) + 1 < this->size(AXIS_Z)) return new Point(curPos, DIR_Z_PLUS);
			break;
		case DIR_Z_MINUS:
			if((curPos->z) - 1 < 0) return new Point(curPos, DIR_Z_MINUS);
			break;
		case DIR_NONE: break;
	}
	return nullptr;
}

Point** PipeLayer::getNeighbors(Point* pos) {
	Point** neighbors = new Point*[NUM_DIRS];
	neighbors[DIR_X_PLUS] = getNextNodePos(pos, DIR_X_PLUS);
	neighbors[DIR_X_MINUS] = getNextNodePos(pos, DIR_X_MINUS);
	neighbors[DIR_Y_PLUS] = getNextNodePos(pos, DIR_Y_PLUS);
	neighbors[DIR_Y_MINUS] = getNextNodePos(pos, DIR_Y_MINUS);
	neighbors[DIR_Z_PLUS] = getNextNodePos(pos, DIR_Z_PLUS);
	neighbors[DIR_Z_MINUS] = getNextNodePos(pos, DIR_Z_MINUS);
	return neighbors;
}

int PipeLayer::getEmptyNeighbors(Point* pos, Direction* emptyDirs) {
	int count = 0;

	for(int i = 0; i < NUM_DIRS; i++) {
		if(getNextNodePos(pos, (Direction) i)) { emptyDirs[count++] = (Direction) i; }
	}
	return count;
}

int PipeLayer::getEmptyTurnNeighbors(Point* pos, Direction* emptyDirs, Direction lastDir) {
	int count = 0;

	for(int i = 0; i < NUM_DIRS; i++) {
		if(getNextNodePos(pos, (Direction) i)) {
			if(i == (int) lastDir) continue;
			emptyDirs[count++] = (Direction) i;
		}
	}
	return count;
}

Direction PipeLayer::chooseRandomDirection(Point* pos, Direction dir, int weight) {
	Point** neighbors = getNeighbors(pos);
	int numEmpty, choice;
	Direction newDir;
	Point* straightNode = NULL;
	Direction* emptyDirs = new Direction;

	// Get node in straight direction if necessary
	if(weight && neighbors[dir] && isEmpty(neighbors[dir])) {
		straightNode = neighbors[dir];
		// if maximum weight, choose and return
		if(weight == MAX_WEIGHT_STRAIGHT) { return dir; }
	} else
		weight = 0;

	// Get directions of possible turns
	numEmpty = getEmptyTurnNeighbors(pos, emptyDirs, dir);

	// Make a random choice
	if((choice = (weight + numEmpty)) == 0) return DIR_NONE;
	choice = iRand(choice);

	if(choice < weight) {
		return dir;
	} else {
		// choose one of the turns
		newDir = emptyDirs[choice - weight];
		return newDir;
	}
}