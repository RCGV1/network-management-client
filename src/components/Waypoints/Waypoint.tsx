import React from "react";
import { useDispatch, useSelector } from "react-redux";
import { Marker } from "react-map-gl";

import type { Waypoint } from "@bindings/protobufs/Waypoint";

import { deviceSliceActions } from "@features/device/deviceSlice";
import { selectActiveWaypointID } from "@features/device/deviceSelectors";

import WaypointIcon from "@app/components/Waypoints/WaypointIcon";

// This component returns a marker for each individual waypoint. It is called from MapView.tsx
interface IWaypoints {
  currWaypoint: Waypoint;
}

const Waypoints = ({ currWaypoint }: IWaypoints) => {
  const dispatch = useDispatch();
  const activeWaypointID = useSelector(selectActiveWaypointID());
  const isSelected = currWaypoint.id === activeWaypointID;

  const expired = currWaypoint.expire < Date.now() / 1000; // Boolean

  // Set current waypoint ID as active if it's not already;
  // Otherwise turn off active if it is already active.
  const handleClick = () => {
    dispatch(
      deviceSliceActions.setActiveWaypoint(!isSelected ? currWaypoint.id : null)
    );
  };

  return (
    <div className="">
      <Marker
        latitude={currWaypoint.latitudeI / 1e7}
        longitude={currWaypoint.longitudeI / 1e7}
        anchor="center"
        onClick={handleClick}
      >
        <WaypointIcon isSelected={isSelected} expired={expired}></WaypointIcon>
      </Marker>
    </div>
  );
};

export default Waypoints;